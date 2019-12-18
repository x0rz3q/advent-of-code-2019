use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Hash, Copy, Clone, Debug)]
struct Coordinate {
	x: i32,
	y: i32,
}

impl Coordinate {
	fn new(x: i32, y: i32) -> Coordinate {
		Coordinate { x: x, y: y }
	}

	fn to_string(&self) -> String {
		format!("{}_{}", self.x, self.y)
	}

	fn get_neighbors(&self) -> Vec<Coordinate> {
		vec![
			Coordinate::new(self.x - 1, self.y),
			Coordinate::new(self.x + 1, self.y),
			Coordinate::new(self.x, self.y - 1),
			Coordinate::new(self.x, self.y + 1),
		]
	}
}

impl PartialEq for Coordinate {
	fn eq(&self, other: &Coordinate) -> bool {
		self.x == other.x && self.y == other.y
	}
}

impl Eq for Coordinate {}

#[derive(Hash, Copy, Clone, Debug)]
struct QueueItem {
	coordinate: Coordinate,
	distance: u64,
}

impl QueueItem {
	fn new(coord: Coordinate, distance: u64) -> QueueItem {
		QueueItem {
			coordinate: coord,
			distance: distance,
		}
	}
}

impl PartialEq for QueueItem {
	fn eq(&self, other: &QueueItem) -> bool {
		self.coordinate.eq(&other.coordinate) && self.distance == other.distance
	}
}

impl Eq for QueueItem {}

#[derive(Hash, Copy, Clone, Debug)]
struct BFSResult {
	from: Coordinate,
	to: Coordinate,
	key: char,
	distance: u64,
}

impl BFSResult {
	fn new(from: Coordinate, to: Coordinate, distance: u64, key: char) -> BFSResult {
		BFSResult {
			from: from,
			to: to,
			distance: distance,
			key: key,
		}
	}
}

#[derive(Hash, Copy, Clone, Debug)]
struct Robot {
	id: usize,
	position: Coordinate,
}

impl Robot {
	fn new(id: usize, position: Coordinate) -> Robot {
		Robot {
			id: id,
			position: position,
		}
	}
}

#[derive(Clone, Debug)]
struct Cluster {
	map: HashMap<Coordinate, char>,
	robots: Vec<Robot>,
	history: HashMap<String, u64>,
	unlocked: HashSet<char>,
}

impl Cluster {
	fn new(map: HashMap<Coordinate, char>, robots: Vec<Robot>) -> Cluster {
		Cluster {
			map: map,
			robots: robots,
			history: HashMap::new(),
			unlocked: HashSet::new(),
		}
	}
}

fn bfs(
	start: Coordinate,
	map: HashMap<Coordinate, char>,
	unlocked: HashSet<char>,
) -> Vec<BFSResult> {
	let mut result = Vec::new();
	let mut queue = VecDeque::new();
	queue.push_back(QueueItem::new(start, 0));
	let mut history = HashSet::new();

	while !queue.is_empty() {
		let item = queue.pop_front().unwrap();

		for neighbor in item.coordinate.get_neighbors() {
			let character = match map.get(&neighbor) {
				Some(e) => *e,
				None => '#',
			};

			if character == '#' || history.contains(&neighbor) {
				continue;
			}

			let distance = item.distance + 1;
			history.insert(neighbor);

			if character.is_uppercase() && !unlocked.contains(&character.to_ascii_lowercase()) {
				continue;
			}

			if character.is_lowercase() && !unlocked.contains(&character) {
				result.push(BFSResult::new(start, neighbor, distance, character));
			} else {
				queue.push_back(QueueItem::new(neighbor, distance));
			}
		}
	}

	result
}

fn storage_string(robots: Vec<Robot>, unlocked: HashSet<char>) -> String {
	let mut sorted = Vec::new();

	for i in unlocked {
		sorted.push(i);
	}

	sorted.sort();

	let mut output = String::from("");

	for i in robots {
		output.push_str(i.position.to_string().as_ref());
	}

	for i in sorted {
		output.push_str(i.to_string().as_ref());
	}

	output
}

fn walk(mut cluster: Cluster) -> (u64, HashMap<String, u64>) {
	let s = storage_string(cluster.robots.clone(), cluster.unlocked.clone());
	if cluster.history.contains_key(&s) {
		return (*cluster.history.get(&s).unwrap(), cluster.history);
	}

	/* get all keys */
	let mut keys = Vec::new();
	for i in cluster.robots.clone() {
		keys.insert(
			i.id,
			bfs(i.position, cluster.map.clone(), cluster.unlocked.clone()),
		);
	}

	let mut done = true;
	for key in keys.clone() {
		done = key.len() == 0 && done;
	}

	if done {
		return (0, cluster.history);
	}

	let mut distance = u64::max_value();
	for robot in cluster.robots.clone() {
		for key in keys[robot.id].clone() {
			let mut c = cluster.clone();
			c.robots[robot.id].position = key.to;
			c.unlocked.insert(key.key);

			let (d, hist) = walk(c);

			cluster.history = hist;

			if distance > d + key.distance {
				distance = d + key.distance;
			}
		}
	}

	cluster.history.insert(
		storage_string(cluster.robots.clone(), cluster.unlocked.clone()),
		distance,
	);

	(distance, cluster.history)
}

fn rewrite_input(
	position: Coordinate,
	mut map: HashMap<Coordinate, char>,
) -> (HashMap<Coordinate, char>, Vec<Robot>) {
	map.insert(
		Coordinate {
			x: position.x,
			y: position.y,
		},
		'#',
	);
	map.insert(
		Coordinate {
			x: position.x + 1,
			y: position.y,
		},
		'#',
	);
	map.insert(
		Coordinate {
			x: position.x - 1,
			y: position.y,
		},
		'#',
	);
	map.insert(
		Coordinate {
			x: position.x,
			y: position.y + 1,
		},
		'#',
	);
	map.insert(
		Coordinate {
			x: position.x,
			y: position.y - 1,
		},
		'#',
	);

	let mut robots = Vec::new();
	let mut id = 0;
	for i in -1..2 {
		for j in -1..2 {
			if i == 0 || j == 0 {
				continue;
			}

			robots.insert(
				id,
				Robot {
					id: id,
					position: Coordinate {
						x: position.x + i,
						y: position.y + j,
					},
				},
			);
			id += 1;
		}
	}

	for robot in robots.clone() {
		map.insert(robot.position.clone(), '@');
	}

	(map, robots)
}

fn main() {
	let input: Vec<char> = include_str!("input").trim().chars().collect();

	let mut map: HashMap<Coordinate, char> = HashMap::new();
	let mut x = 0;
	let mut y = 0;
	let mut robots = Vec::new();
	let mut id = 0;

	for i in input {
		if i == '\n' {
			x = 0;
			y += 1;
		} else {
			if i == '@' {
				robots.push(Robot::new(id, Coordinate::new(x, y)));
				id += 1;
			}

			map.insert(Coordinate::new(x, y), i);
			x += 1;
		}
	}

	println!(
		"Silver: {}",
		walk(Cluster::new(map.clone(), robots.clone())).0
	);
	let position = robots[0].position;
	let (map, robots) = rewrite_input(position, map);
	println!(
		"Gold: {}",
		walk(Cluster::new(map.clone(), robots.clone())).0
	);
}
