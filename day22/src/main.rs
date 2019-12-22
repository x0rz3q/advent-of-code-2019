extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigInt;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::Add;
use num::integer::mod_floor;
use num::integer::div_floor;

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

fn matrix_multiplication() {
}

/**
 * We want to calculate this with algebra, otherwise the running time would be crazy.
 * If we start thinking about what happens we get the following, we have a multiplier and an
 * offset, that gets changed in every step. Which means we could write this as an equation,
 * which means we have ax+b. Hence, we initialize a = 1 and b = 0 in the beginning. Simply
 * representing the index of each element. When we see a cut we want to offset all items by the
 * amount given, hence we get b = b + cut % modulus, when we see a deal into we need to flip the entire
 * structure, which means we need to flip the multiplier so a = a * -1, but we also need to make
 * sure our offset is still handled correctly so b = -b - 1, flipping it. When we see a deal with
 * increments we get that everything is increased by a certain number, so we get a = a * inverse(increment),
 * b = b * inverse(increment). Simply, multiplying everything with this. Everything is modulus the amount of
 * cards, to make sure we do not go out of bounds.
 *
 * We then want to find a * x + b = 2020 mod modulus.
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
			if offset < 0 {
				b = b.add(modulus);
			}
		} else {
			let parts: Vec<String> = rule.split(" ").map(|x| x.to_string()).collect();
			let offset = parts[3].to_string().parse::<i64>().unwrap();
			let inverse = inverse(BigInt::from(offset), BigInt::from(modulus));

			a = a.mul(inverse.clone());
			b = b.mul(inverse);
		}

		a = mod_floor(a, BigInt::from(modulus));
		b = mod_floor(b, BigInt::from(modulus));
	}

	a = mod_floor(a, BigInt::from(modulus));
	b = mod_floor(b, BigInt::from(modulus));

	let mut theta = a.modpow(&BigInt::from(repeat), &BigInt::from(modulus));
	theta = theta.mul(2020);
	theta = mod_floor(theta, BigInt::from(modulus));

	let mut phi = a.modpow(&BigInt::from(repeat), &BigInt::from(modulus));
	phi = phi.add(modulus - 1);
	phi = mod_floor(phi, BigInt::from(modulus));

	let mut lambda = b.mul(phi);
	lambda = mod_floor(lambda, BigInt::from(modulus));

	let mut omega: BigInt = a.sub(1);
	omega = inverse(omega.clone(), BigInt::from(modulus));
	
	let mut result = lambda.mul(omega);
	result = result.add(theta);
	result = mod_floor(result, BigInt::from(modulus));

	result
}

fn main() {
	assert_eq!(cut(vec![0,1,2,3,4,5,6,7,8,9], 3), vec![3,4,5,6,7,8,9,0,1,2]);
	assert_eq!(cut(vec![0,1,2,3,4,5,6,7,8,9], -4), vec![6,7,8,9,0,1,2,3,4,5]);
	assert_eq!(deal_new_deck(vec![0,1,2,3]), vec![3,2,1,0]);
	assert_eq!(deal_increment(vec![0,1,2,3,4,5,6,7,8,9], 3), vec![0,7,4,1,8,5,2,9,6,3]);

	let rules: Vec<String> = include_str!("input")
		.trim()
		.split('\n')
		.map(|x| x.to_string())
		.collect();

	println!("Silver: {}", silver(rules.clone()));
	println!("Gold: {}", gold(rules.clone()));
}
