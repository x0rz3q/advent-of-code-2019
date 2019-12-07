fn is_increasing(number: usize) -> bool {
	let mut number = number;
	let mut last = usize::max_value();

	while number > 0 {
		let digit = number % 10;
		if last < digit {
			return false;
		}

		last = digit;
		number = number / 10;
	}

	return true;
}

fn has_pattern(number: usize, min: usize, max: usize) -> bool {
	let mut count = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
	let mut number = number;

	while number > 0 {
		let digit = number % 10;
		count[digit] += 1;

		number = number / 10;
	}

	for i in count {
		if i >= min && i <= max {
			return true;
		}
	}

	false
}

fn main() {
	let mut count = 0;
	for i in 134564..585159 + 1 {
		if is_increasing(i) && has_pattern(i, 2, 10) {
			count += 1;
		}
	}
	println!("Silver: {}", count);

	let mut count = 0;
	for i in 134564..585159 + 1 {
		if is_increasing(i) && has_pattern(i, 2, 2) {
			count += 1;
		}
	}
	println!("Gold: {}", count);
}
