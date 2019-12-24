use std::collections::HashSet;

#[derive(Clone, Hash, Debug, Copy)]
struct Bug {
	x: i64,
	y: i64,
	z: i64,
}

impl Bug {
	fn new(x: i64, y: i64, z: i64) -> Bug {
		Bug { x, y, z }
	}

	fn left(&self) -> Bug {
		Bug {
			x: self.x - 1,
			y: self.y,
			z: self.z,
		}
	}

	fn right(&self) -> Bug {
		Bug {
			x: self.x + 1,
			y: self.y,
			z: self.z,
		}
	}

	fn above(&self) -> Bug {
		Bug {
			x: self.x,
			y: self.y - 1,
			z: self.z,
		}
	}

	fn below(&self) -> Bug {
		Bug {
			x: self.x,
			y: self.y + 1,
			z: self.z,
		}
	}

	fn neighbors(&self) -> Vec<Bug> {
		vec![self.left(), self.right(), self.above(), self.below()]
	}

	fn multilevel_neighbors(&self) -> Vec<Bug> {
		let mut neighbors = self.neighbors();

		if self.y == 4 {
			neighbors.push(Bug::new(2, 3, self.z - 1));
		}

		if self.y == 0 {
			neighbors.push(Bug::new(2, 1, self.z - 1));
		}

		if self.x == 0 {
			neighbors.push(Bug::new(1, 2, self.z - 1));
		}

		if self.x == 4 {
			neighbors.push(Bug::new(3, 2, self.z - 1));
		}

		if self.x == 1 && self.y == 2 {
			for y in 0..5 {
				neighbors.push(Bug::new(0, y, self.z + 1));
			}
		}

		/* handle center cells, 14 in the diagram */
		if self.x == 3 && self.y == 2 {
			for y in 0..5 {
				neighbors.push(Bug::new(4, y, self.z + 1));
			}
		}

		/* handle center cells, 8 in the diagram */
		if self.x == 2 && self.y == 1 {
			for x in 0..5 {
				neighbors.push(Bug::new(x, 0, self.z + 1));
			}
		}

		/* handle center cells, 18 in the diagram */
		if self.x == 2 && self.y == 3 {
			for x in 0..5 {
				neighbors.push(Bug::new(x, 4, self.z + 1));
			}
		}

		let neighbors = neighbors
			.into_iter()
			.filter(|bug| !(bug.x == 2 && bug.y == 2))
			.collect();
		neighbors
	}
}

impl Eq for Bug {}

impl PartialEq for Bug {
	fn eq(&self, other: &Bug) -> bool {
		self.x == other.x && self.y == other.y && self.z == other.z
	}
}

fn silver(bugs: Vec<Bug>) -> u64 {
	let mut bugs = bugs;
	let mut history: HashSet<Vec<Bug>> = HashSet::new();
	history.insert(bugs.clone());

	loop {
		let mut refresh: Vec<Bug> = Vec::new();

		for y in 0..5 {
			for x in 0..5 {
				let bug = Bug { x: x, y: y, z: 0 };
				let neighbors = bug.neighbors();
				let count = neighbors
					.iter()
					.filter(|x| bugs.contains(x) && !bug.eq(x))
					.count();

				if bugs.contains(&bug) && count == 1 {
					refresh.push(bug);
				} else if count > 0 && count <= 2 && !bugs.contains(&bug) {
					refresh.push(bug);
				}
			}
		}

		bugs = refresh.clone();
		if history.contains(&bugs.clone()) {
			break;
		} else {
			history.insert(bugs.clone());
		}
	}

	let mut sum: u64 = 0;
	let mut index = 0;
	let base: u64 = 2;
	for y in 0..5 {
		for x in 0..5 {
			let bug = Bug { x, y, z: 0 };

			if bugs.contains(&bug) {
				sum += base.pow(index);
			}
			index += 1;
		}
	}

	sum
}

fn gold(bugs: Vec<Bug>) {
	let mut bugs = bugs;
	for _ in 0..200 {
		let min = bugs.iter().min_by_key(|x| x.z).unwrap().z;
		let max = bugs.iter().max_by_key(|x| x.z).unwrap().z;
		let mut refresh: Vec<Bug> = Vec::new();

		for z in (min - 1)..(max + 2) {
			for x in 0..5 {
				for y in 0..5 {
					if x == 2 && y == 2 {
						continue;
					}

					let bug = Bug { x: x, y: y, z: z };
					let neighbors = bug.multilevel_neighbors();
					let count = neighbors
						.iter()
						.filter(|x| bugs.contains(x) && !bug.eq(x))
						.count();

					if bugs.contains(&bug) && count == 1 {
						refresh.push(bug);
					} else if count > 0 && count <= 2 && !bugs.contains(&bug) {
						refresh.push(bug);
					}
				}
			}
		}

		bugs = refresh.clone();
	}

	println!("{}", bugs.len());
}

fn main() {
	let input: Vec<String> = include_str!("input")
		.trim()
		.split("\n")
		.map(|x| x.to_string())
		.collect();

	let mut bugs: Vec<Bug> = Vec::new();
	let mut x = 0;
	let mut y = 0;
	for line in input {
		for ch in line.chars() {
			if ch == '#' {
				bugs.push(Bug::new(x, y, 0));
			}
			x += 1;
		}

		y += 1;
		x = 0;
	}

	println!("Silver: {}", silver(bugs.clone()));
	gold(bugs);
}
