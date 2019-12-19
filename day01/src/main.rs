fn calculate_fuel(input: i64) -> i64 {
	(input / 3) - 2
}

fn calculate_fuel_rec(input: i64) -> i64 {
	let n = calculate_fuel(input);
	if n <= 0 {
		return 0;
	}

	n + calculate_fuel_rec(n)
}

fn main() {
	let input: Vec<i64> = include_str!("input")
		.trim()
		.split('\n')
		.map(|num| num.parse::<i64>().unwrap())
		.collect();

	println!(
		"Silver {}",
		input
			.iter()
			.map(|num| calculate_fuel(*num))
			.fold(0i64, |r, s| r + s)
	);

	println!(
		"Gold {}",
		input
			.iter()
			.map(|num| calculate_fuel_rec(*num))
			.fold(0i64, |r, s| r + s)
	);
}
