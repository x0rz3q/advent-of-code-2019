use std::cmp::Ordering::Equal;
use std::collections::HashSet;

fn calculate(x: usize, y: usize, map: Vec<Vec<bool>>) -> usize {
	let mut angles: Vec<f64> = Vec::new();

	for i in 0..map.len() {
		for j in 0..map[0].len() {
			if !map[i][j] {
				continue;
			}

			if x == j && y == i {
				continue;
			}

			let angle = (y as f64 - i as f64)
				.atan2(x as f64 - j as f64)
				.to_degrees();
			let angle = (angle * 100000.0) / 100000.0;

			if !angles.contains(&angle) {
				angles.push(angle);
			}
		}
	}

	return angles.len();
}

fn dist(x: usize, y: usize, x1: usize, y1: usize) -> f64 {
	let x = x as f64;
	let y = y as f64;
	let x1 = x1 as f64;
	let y1 = y1 as f64;

	return ((x - x1) * (x - x1) + (y - y1) * (y - y1)).sqrt();
}

struct Astroid {
	dist: f64,
	x: usize,
	y: usize,
	angle: f64,
}

fn vaporize(x: usize, y: usize, map: Vec<Vec<bool>>) -> usize {
	let mut blacklist_x: HashSet<usize> = HashSet::new();
	let mut blacklist_y: HashSet<usize> = HashSet::new();
	let mut destroyed = 0;

	loop {
		let mut angles: Vec<f64> = Vec::new();
		let mut astroids: Vec<Astroid> = Vec::new();

		for i in 0..map.len() {
			for j in 0..map[0].len() {
				if !map[i][j] {
					continue;
				}

				if x == j && y == i {
					continue;
				}

				if blacklist_x.contains(&j) && blacklist_y.contains(&i) {
					continue;
				}

				let angle = (y as f64 - i as f64)
					.atan2(x as f64 - j as f64)
					.to_degrees();
				let d = dist(x, y, j, i);

				if angles.contains(&angle) {
					let pos = angles.iter().position(|&r| r == angle).unwrap();
					let astroid = &astroids[pos];
					if astroid.dist > d {
						astroids[pos] = Astroid {
							x: j,
							y: i,
							dist: d,
							angle: (angle + 270.0 + 360.0) % 360.0,
						};
					}
				} else {
					angles.push(angle);
					astroids.push(Astroid {
						x: j,
						y: i,
						dist: d,
						angle: (angle + 270.0 + 360.0) % 360.0,
					});
				}
			}
		}

		astroids.sort_by(|a, b| a.angle.partial_cmp(&b.angle).unwrap_or(Equal));

		for astroid in astroids {
			blacklist_x.insert(astroid.x);
			blacklist_y.insert(astroid.y);

			destroyed += 1;

			if destroyed == 200 {
				return astroid.x * 100 + astroid.y;
			}
		}
	}
}

fn main() {
	let input: Vec<String> = include_str!("input")
		.trim()
		.split('\n')
		.map(|line| line.to_string())
		.collect();

	let mut map: Vec<Vec<bool>> = Vec::new();
	for line in input {
		let row: Vec<bool> = line.chars().map(|x| x == '#').collect();
		map.push(row);
	}

	let width = map[0].len();
	let height = map.len();
	let mut count: Vec<usize> = Vec::new();

	let mut x_best = 0;
	let mut y_best = 0;
	let mut best = 0;
	for y in 0..height {
		for x in 0..width {
			if map[y][x] {
				let result = calculate(x, y, map.clone());
				if result > best {
					count.push(result);
					x_best = x;
					y_best = y;

					best = result;
				}
			}
		}
	}

	let m = count.iter().max().unwrap();
	println!("Silver: {}", m);
	let result = vaporize(x_best, y_best, map.clone());
	println!("Gold: {}", result);
}
