use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

fn count(mapping: &HashMap<String, Vec<String>>, target: String) -> u64 {
    let mut c: u64 = 0;

    if !mapping.contains_key(&target) {
        return 0;
    }

    let sources = mapping.get(&target).unwrap().to_vec();
    c += sources.len() as u64;

    for source in sources {
        c += count(mapping, source);
    }

    c
}

/// Just a recursion
fn silver(input: Vec<String>) -> u64 {
    let mut mapping: HashMap<String, Vec<String>> = HashMap::new();
    for i in input {
        let split: Vec<String> = i.split(')').map(|item| String::from(item)).collect();
        let from = split[0].to_string();
        let to = split[1].to_string();

        if mapping.contains_key(&to) {
            let mut orbits: Vec<String> = mapping.get(&to).unwrap().to_vec();
            orbits.push(from.to_string());
        } else {
            let mut orbits: Vec<String> = Vec::new();
            orbits.push(from.to_string());
            mapping.insert(to.to_string(), orbits);
        }
    }

    let mut c = 0;
    for (source, _targets) in mapping.clone() {
        c += count(&mapping.clone(), source.to_string());
    }

	c	
}

/// Simple BFS like solution
fn gold(input: Vec<String>) -> u64 {
	let mut mapping: HashMap<String, Vec<String>> = HashMap::new();

    for i in input {
        let split: Vec<String> = i.split(')').map(|item| String::from(item)).collect();
        let from = split[0].to_string();
        let to = split[1].to_string();

        if mapping.contains_key(&to) {
            let mut orbits: Vec<String> = mapping.get(&to).unwrap().to_vec();
            orbits.push(from.to_string());
			mapping.insert(to.to_string(), orbits);
        } else {
            let mut orbits: Vec<String> = Vec::new();
            orbits.push(from.to_string());
            mapping.insert(to.to_string(), orbits);
        }

        if mapping.contains_key(&from) {
            let mut orbits: Vec<String> = mapping.get(&from).unwrap().to_vec();
            orbits.push(to.to_string());
			mapping.insert(from.to_string(), orbits);
        } else {
            let mut orbits: Vec<String> = Vec::new();
            orbits.push(to.to_string());
            mapping.insert(from.to_string(), orbits);
        }
    }

	/* BFS */
	let mut queue = VecDeque::new();
	let mut history: HashSet<String> = HashSet::new();
	queue.push_front(("YOU".to_string(), 0));

	while !queue.is_empty() {
		let item = queue.pop_front().unwrap();
		let name = item.0;
		let distance = item.1;

		if history.contains(&name) {
			continue;
		}

		history.insert(name.clone());

		if name == "SAN".to_string() {
			return distance - 2;
		}

		for i in mapping.get(&name).unwrap() {
			queue.push_back((i.to_string(), distance + 1));
		}
	}

	0
}

fn main() {
    let input: Vec<String> = include_str!("input")
        .trim()
        .split('\n')
        .map(|row| String::from(row))
        .collect();

    println!("Silver: {}", silver(input.clone()));
	println!("Gold: {}", gold(input.clone()));
}
