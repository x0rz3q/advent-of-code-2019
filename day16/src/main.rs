use std::time::{Duration, Instant};

fn get_pattern(length: usize, repeat: usize) -> Vec<i64> {
	let mut sequence: Vec<i64> = Vec::new();
	let pattern = vec![0, 1, 0, -1];

	'outer: while sequence.len() <= length + 1 {
		for i in 0..pattern.len() {
			for j in 0..repeat {
				sequence.push(pattern[i]);

				if sequence.len() == length + 1 {
					break 'outer;
				}
			}
		}
	}

	sequence.remove(0);
	sequence
}

fn phase(input: Vec<i64>) -> Vec<i64> {
	let mut output: Vec<i64> = Vec::new();

	let mut iteration = 1;
	let mut k = 1;
	for _ in 0..input.len() {
		let mut digit: i64 = 0;
		let pattern = get_pattern(input.len(), k);

		for i in 0..input.len() {
			digit += (pattern[i] * input[i]) % 10;
		}

		if digit < 0 {
			digit *= -1;
		}

		output.push(digit % 10);

		k += 1;
		iteration += 1;
	}

	output
}

fn silver(input: Vec<i64>) -> i64 {
	let mut input = input.clone();
	for i in 0..100 {
		input = phase(input);
	}

	let mut output = 0;
	for i in 0..8 {
		output *= 10;
		output += input[i];
	}

	output
}

/**
 * The last digit always stays the same.
 * Then for the rest we have the following function
 *
 *        | k[i]					if i == length(k)
 * k[i] = | k[i] + k[i + 1] mod 10	if i >= length(k) / 2 and i < length(k)
 *        | undefined				if i < length(k)
 *
 * Keep in mind, this only applies for the lower half of the array.
 * It seems that our 8 digit 5977377 > ceil(650000 / 2),
 * which means we can use this property.
 */
fn gold(input: Vec<i64>) -> i64 {
	let (a, _) = input.as_slice().split_at(7);
	let mut input = input.clone();

	let mut offset = 0;
	for i in a {
		offset *= 10;
		offset += i;
	}

	let offset = offset as usize;

	for _ in 0..100 {
		for i in (offset..input.len() - 1).rev() {
			input[i] = (input[i] + input[i + 1]) % 10;
		}
	}

	let mut output = 0;
	for i in 0..8 {
		output *= 10;
		output += input[i + offset];
	}

	output
}

fn main() {
	let mut input: Vec<i64> = include_str!("input")
		.trim()
		.chars()
		.map(|num| num.to_digit(10).unwrap() as i64)
		.collect();

	let mut repeated: Vec<i64> = Vec::new();
	for i in 0..10000 {
		repeated.append(&mut input.clone());
	}

	let output = silver(input.clone());
	println!("Silver: {}", output);
	println!("Gold: {}", gold(repeated));
}
