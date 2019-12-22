extern crate num_bigint;
extern crate num_traits;

use num::integer::div_floor;
use num::integer::mod_floor;
use num_bigint::BigInt;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Clone, Debug)]
struct Function {
	a: BigInt,
	b: BigInt,
	modulus: BigInt,
}

impl Function {
	fn new(a: BigInt, b: BigInt, modulus: BigInt) -> Function {
		Function { a, b, modulus }
	}

	fn compose(&self, other: Function) -> Function {
		let mut z = self.a.clone().mul(other.a.clone());
		z = mod_floor(z, self.modulus.clone());
		let mut q = other.b.mul(self.a.clone());
		q = q.add(self.b.clone());
		q = mod_floor(q, self.modulus.clone());

		Function::new(z, q, self.modulus.clone())
	}

	fn solve(&self, x: u64) -> BigInt {
		let mut result = self.a.clone();
		result = result.mul(x);
		result = result.add(self.b.clone());
		result = mod_floor(result, self.modulus.clone());

		result
	}
}

fn cut(cards: Vec<i64>, n: i64) -> Vec<i64> {
	let mut n = n;
	let mut cards = cards;

	if n < 0 {
		n += cards.len() as i64;
	}

	let mut split = cards.split_off(n as usize);
	split.append(&mut cards);

	split
}

fn deal_new_deck(cards: Vec<i64>) -> Vec<i64> {
	let mut cards = cards;
	cards.reverse();
	cards
}

fn deal_increment(cards: Vec<i64>, n: usize) -> Vec<i64> {
	let mut output = vec![0; cards.len()];

	let mut index = 0;
	for card in cards.clone() {
		output[index] = card;
		index = (index + n) % cards.len();
	}

	output
}

fn step(cards: Vec<i64>, rules: Vec<String>) -> Vec<i64> {
	let mut cards = cards;

	for rule in rules {
		if rule.contains("deal into new stack") {
			cards = deal_new_deck(cards);
		} else if rule.contains("cut") {
			let parts: Vec<String> = rule.split(" ").map(|x| x.to_string()).collect();
			let offset = parts[1].to_string().parse::<i64>().unwrap();
			cards = cut(cards.clone(), offset);
		} else {
			let parts: Vec<String> = rule.split(" ").map(|x| x.to_string()).collect();
			let offset = parts[3].to_string().parse::<i64>().unwrap();
			cards = deal_increment(cards.clone(), offset as usize);
		}
	}

	cards
}

fn silver(rules: Vec<String>) -> i64 {
	let mut cards: Vec<i64> = (0..10007).collect();
	cards = step(cards.clone(), rules.clone());

	cards.iter().position(|&x| x == 2019).unwrap() as i64
}

/**
 * https://www.geeksforgeeks.org/multiplicative-inverse-under-modulo-m/
 */
fn inverse(a: BigInt, m: BigInt) -> BigInt {
	let mut m = m.clone();
	let mut m0 = m.clone();
	let mut y = BigInt::from(0);
	let mut x = BigInt::from(1);
	let mut a = a.clone();

	while a.clone().gt(&BigInt::from(1)) {
		let mut q = div_floor(a.clone(), m.clone());
		let mut temp = m.clone();
		m = mod_floor(a, m);
		a = temp.clone();
		temp = y.clone();
		y = x.sub(q.mul(y));
		x = temp;
	}

	if x.lt(&BigInt::from(0)) {
		x = x.add(m0);
	}

	x
}

fn construct_function(f: Function, repeat: u64) -> Function {
	if repeat == 0 {
		return Function {
			a: BigInt::from(1),
			b: BigInt::from(0),
			modulus: f.modulus.clone(),
		};
	}

	let is_odd = repeat % 2 == 1;
	let middle = repeat / 2;

	let g = construct_function(f.clone(), middle);
	let gog = g.compose(g.clone());

	if !is_odd {
		return gog;
	}

	gog.compose(f.clone())
}

/**
 * We want to calculate this with algebra, otherwise the running time would be crazy.
 * We first observe that we have an offset and a multiplier, that gets changed by each "rule" of
 * shuffle we need to follow. Which means, we can write this as an equation ax+b. We want to apply
 * these rules backwards, because we are interested in the inverse. So, if we encounter a new we
 * simply do b' = b*-1 - 1, a' = a * -1, if we encounter cut we simply have b' = b + offset, and if
 * we encounter a increment we multiply a and b by the inverse of the given offset.
 *
 * We then want to after repeating (ax+b) 101741582076661 times, we can solve this in multiple
 * ways. We could do this with modular arithmetic or by repeating the inverse function
 * 101741582076661 times. Personally, I tried to do it both ways as can be seen in the git history
 * but I think the repeating inverse function results in cleaner code, that is easier to explain.
 * After repeating this 101741582076661 times we simply need to fill out our x and we have what we
 * need. Repeating in this context means the following, f(f(f(....))), basically using the compose
 * rules e.g. fog(x). We then can solve it by f(f(f(...)))(2020), which will yield the correct
 * result.
 */
fn gold(rules: Vec<String>) -> BigInt {
	let modulus: u64 = 119315717514047;
	let repeat: u64 = 101741582076661;
	let mut a = BigInt::from(1);
	let mut b = BigInt::from(0);

	for rule in rules.iter().rev() {
		if rule.contains("new") {
			b = b.mul(-1);
			b = b.sub(1);
			a = a.mul(-1);
		} else if rule.contains("cut") {
			let parts: Vec<String> = rule.split(" ").map(|x| x.to_string()).collect();
			let offset = parts[1].to_string().parse::<i64>().unwrap();
			b = b.add(offset);
		} else {
			let parts: Vec<String> = rule.split(" ").map(|x| x.to_string()).collect();
			let offset = parts[3].to_string().parse::<i64>().unwrap();
			let inverse = inverse(BigInt::from(offset), BigInt::from(modulus));

			a = a.mul(inverse.clone());
			b = b.mul(inverse);
		}
	}

	a = mod_floor(a, BigInt::from(modulus));
	b = mod_floor(b, BigInt::from(modulus));
	let f = Function {
		a: a.clone(),
		b: b.clone(),
		modulus: BigInt::from(modulus),
	};

	let composition = construct_function(f, repeat);
	composition.solve(2020)
}

fn main() {
	assert_eq!(
		cut(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3),
		vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]
	);
	assert_eq!(
		cut(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], -4),
		vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]
	);
	assert_eq!(deal_new_deck(vec![0, 1, 2, 3]), vec![3, 2, 1, 0]);
	assert_eq!(
		deal_increment(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 3),
		vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]
	);

	let rules: Vec<String> = include_str!("input")
		.trim()
		.split('\n')
		.map(|x| x.to_string())
		.collect();

	println!("Silver: {}", silver(rules.clone()));
	println!("Gold: {}", gold(rules.clone()));
}
