use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

/* simple coordinate definition. */
#[derive(Hash, Clone, Debug, Copy)]
struct Coordinate {
	x: i64,
	y: i64,
}

impl Coordinate {
	fn new(x: i64, y: i64) -> Coordinate {
		Coordinate { x, y }
	}

	fn left(&self) -> Coordinate {
		Coordinate {
			x: self.x - 1,
			y: self.y,
		}
	}

	fn right(&self) -> Coordinate {
		Coordinate {
			x: self.x + 1,
			y: self.y,
		}
	}

	fn up(&self) -> Coordinate {
		Coordinate {
			x: self.x,
			y: self.y - 1,
		}
	}

	fn down(&self) -> Coordinate {
		Coordinate {
			x: self.x,
			y: self.y + 1,
		}
	}

	fn neighbors(&self) -> Vec<Coordinate> {
		vec![self.left(), self.right(), self.up(), self.down()]
	}
}

impl Eq for Coordinate {}

impl PartialEq for Coordinate {
	fn eq(&self, other: &Coordinate) -> bool {
		return self.x == other.x && self.y == other.y;
	}
}

#[derive(Hash, Clone)]
struct Episode {
	position: Coordinate,
	steps: i64,
	level: i64,
}

impl Episode {
	fn new(position: Coordinate, steps: i64, level: i64) -> Episode {
		Episode {
			position,
			steps,
			level,
		}
	}
}

fn silver(
	map: HashMap<Coordinate, char>,
	begin: Coordinate,
	portals: HashMap<Coordinate, Coordinate>,
) -> i64 {
	/* history, what did we visit already. */
	let mut history: HashSet<Coordinate> = HashSet::new();
	/* what do we still need to explore. */
	let mut episodes: VecDeque<Episode> = VecDeque::new();
	/* push the first starting point. */
	episodes.push_back(Episode::new(begin, 0, 0));

	/* run while no episodes are found. */
	while !episodes.is_empty() {
		let episode = episodes.pop_front().unwrap();
		let mut position = episode.position;
		let mut steps = episode.steps;

		history.insert(episode.position.clone());
		if portals.contains_key(&episode.position) {
			let portal = *portals.get(&episode.position).unwrap();
			steps += 1;
			position = portal;
		}

		for neighbor in position.neighbors() {
			if history.contains(&neighbor) {
				continue;
			}

			let tile = match map.get(&neighbor) {
				Some(e) => *e,
				None => '#',
			};

			match tile {
				'.' => {
					episodes.push_back(Episode::new(neighbor, steps + 1, 0));
				}
				'@' => {
					return steps + 1;
				}
				_ => (),
			}
		}
	}

	0
}

fn gold(
	map: HashMap<Coordinate, char>,
	begin: Coordinate,
	portals: HashMap<Coordinate, Coordinate>,
	width: i64,
	height: i64,
) -> i64 {
	/* history per level. */
	let mut archive: HashMap<i64, HashSet<Coordinate>> = HashMap::new();
	/* episodes, vector now so we can sort. */
	let mut episodes: Vec<Episode> = Vec::new();
	/* push the begin episode. */
	episodes.push(Episode::new(begin, 0, 0));

	while !episodes.is_empty() {
		/* explore first possible lower levels. */
		episodes.sort_by(|x, y| x.level.cmp(&y.level));

		let episode = episodes.remove(0);
		let mut position = episode.position;
		let mut steps = episode.steps + 1;
		let mut level = episode.level;

		/* found portal */
		if portals.contains_key(&position) {
			let portal = *portals.get(&position).unwrap();

			/* outer */
			if position.x < 4 || position.x > width - 4 || position.y < 4 || position.y > height - 4
			{
				/* only allowed when non-zero */
				if level > 0 {
					position = portal;
					level -= 1;
				} else {
					continue;
				}
			} else {
				/* inner */
				position = portal;
				level += 1;
			}

			steps += 1;
		}

		let mut history;
		if archive.contains_key(&level) {
			history = archive.get(&level).unwrap().clone();
		} else {
			history = HashSet::new();
		}

		for neighbor in position.neighbors() {
			if history.contains(&neighbor) {
				continue;
			}

			history.insert(neighbor);
			let tile = match map.get(&neighbor) {
				Some(e) => *e,
				None => '#',
			};

			match tile {
				'.' => {
					episodes.push(Episode::new(neighbor, steps, level));
				}
				'@' => {
					if level == 0 {
						return steps;
					}
				}
				_ => (),
			}
		}

		archive.insert(level, history);
	}

	0
}

fn main() {
	let input: Vec<String> = include_str!("input")
		.split('\n')
		.map(|x| x.to_string())
		.collect();

	let mut map = HashMap::new();
	let mut x = 0;
	let mut width = 0;
	let mut y = 0;
	let height = input.len() as i64;
	let mut begin = Coordinate::new(0, 0);
	let mut letters = HashMap::new();
	/* parse into template */
	for line in input.clone() {
		for c in line.chars() {
			if c == ' ' {
				x += 1;
				continue;
			}

			let mut ch = c;
			if c == '^' {
				begin = Coordinate::new(x, y);
			} else if c.is_uppercase() {
				letters.insert(Coordinate::new(x, y), c);
				ch = '#';
			}

			map.insert(Coordinate::new(x, y), ch);
			x += 1;
		}

		if x > width {
			width = x;
		}

		x = 0;
		y += 1;
	}

	/* make portals */
	let mut portals = HashMap::new();
	let mut portal_store = HashMap::new();

	for (coord, letter) in letters.clone() {
		/* not a start portal */
		if !letters.contains_key(&coord.right()) && !letters.contains_key(&coord.down()) {
			continue;
		}

		/* find full portal name */
		let neighbor;
		let ncoord;
		if letters.contains_key(&coord.right()) {
			neighbor = *letters.get(&coord.right()).unwrap();
			ncoord = coord.right();
		} else {
			neighbor = *letters.get(&coord.down()).unwrap();
			ncoord = coord.down();
		}

		/* find portal dot */
		let mut neighbors = coord.neighbors();
		neighbors.append(&mut ncoord.neighbors());

		let mut stone = Coordinate::new(0, 0);
		for k in neighbors {
			let tile = match map.get(&k) {
				Some(e) => *e,
				None => '#',
			};

			if tile == '.' {
				stone = k;
				break;
			}
		}

		let id = format!("{}{}", letter, neighbor).to_string();
		if id == "AA".to_string() || id == "ZZ".to_string() {
			continue;
		}

		if !portal_store.contains_key(&id) {
			portal_store.insert(id, stone.clone());
		} else {
			let to = portal_store.get(&id).unwrap();
			portals.insert(*to, stone.clone());
			portals.insert(stone.clone(), *to);
		}
	}

	println!("Silver: {}", silver(map.clone(), begin, portals.clone()));
	println!(
		"Gold: {}",
		gold(map.clone(), begin, portals.clone(), width, height)
	);
}
