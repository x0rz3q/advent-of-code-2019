use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::collections::HashSet;

struct Point {
	x: i64,
	y: i64,
	count: i64,
	steps: i64,
}

fn index(x: i64, y: i64) -> String {
	format!("{}_{}", x, y)
}

fn walk(mut points: HashMap<String, Point>, path: Vec<String>) -> HashMap<String, Point> {
	let mut x: i64 = 0;
	let mut y: i64 = 0;
	let mut encounters: HashSet<String> = HashSet::new();
	let mut count = 0;

	for entry in path {
		let steps = entry.get(1..).unwrap().parse::<i64>().unwrap();

		for _ in 0..steps {
			count += 1;
			match entry.get(0..1).unwrap() {
				"R" => {
					x += 1;
				}
				"L" => {
					x -= 1;
				}
				"U" => {
					y += 1;
				}
				"D" => {
					y -= 1;
				}
				_ => {
					panic!("Step type not found");
				}
			};

			let mut point = points.entry(index(x, y)).or_insert(Point {
				x: x,
				y: y,
				count: 0,
				steps: 0,
			});

			if !encounters.contains(&index(x, y)) {
				point.count += 1;
				point.steps += count;

				encounters.insert(index(x, y));
			}
		}
	}

	return points;
}

fn main() {
	let input: Vec<String> = include_str!("input")
		.trim()
		.split("\n")
		.map(|entry| entry.to_string())
		.collect();

	let first = input[0]
		.trim()
		.split(',')
		.map(|entry| entry.to_string())
		.collect();

	let second = input[1]
		.trim()
		.split(',')
		.map(|entry| entry.to_string())
		.collect();

	let points: HashMap<String, Point> = HashMap::new();
	let points = walk(points, first);
	let points = walk(points, second);

	let mut distance = i64::max_value();
	let mut steps = i64::max_value();
	for (_, value) in points {
		if value.count > 1 {
			distance = min(distance, value.x.abs() + value.y.abs());
			steps = min(steps, value.steps);
		}
	}

	println!("Silver: {}", distance);
	println!("Gold: {}", steps);
}
