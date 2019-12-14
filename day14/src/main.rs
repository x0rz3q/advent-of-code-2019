extern crate regex;
use regex::Regex;
use std::collections::HashMap;

struct Ingredients {
	name: String,
	resources: u64
}

struct Recipe {
	name: String,
	output: u64,
	input: Vec<Ingredients>
}

fn parse_item(item: String) -> (String, u64) {
	let parts: Vec<String> = item.split(" ").map(|x| x.trim().to_string()).collect();
	let quantity = parts[0].parse::<u64>().unwrap();

	(parts[1].to_string(), quantity)
}

fn parse_line(input: String) -> Recipe {
	let parts: Vec<String> = input.split(" => ").map(|x| x.trim().to_string()).collect();
	let (name, output) = parse_item(parts[1].to_string());

	let mut recipe = Recipe {
		name: name,
		output: output,
		input: Vec::new()
	};

	let parts: Vec<String> = parts[0].split(",").map(|x| x.trim().to_string()).collect();
	for part in parts {
		let (name, quantity) = parse_item(part.to_string());
		recipe.input.push(Ingredients {
			name: name,
			resources: quantity
		});
	}

	recipe
}

fn parse_input(lines: Vec<String>) -> (HashMap<String, u64>, HashMap<String, Recipe>) {
	let mut wanted: HashMap<String, u64> = HashMap::new();
	let mut cookbook: HashMap<String, Recipe> = HashMap::new();

	for line in lines {
		let recipe = parse_line(line);
		wanted.entry(recipe.name.to_string()).or_insert(0);
		cookbook.insert(recipe.name.to_string(), recipe);
	}

	(wanted, cookbook)
}

fn calculate(cookbook: &HashMap<String, Recipe>, demand: u64) -> u64 {
	/* how much do we still have. */
	let mut resources: HashMap<String, u64> = HashMap::new();
	/* what do we still need to produce. */
	let mut needed: Vec<Ingredients> = Vec::new();
	/* how much ore did we get. */
	let mut ore: u64 = 0;
	/* add the fuel to the beginning of needed. */
	needed.push(Ingredients{name: "FUEL".to_string(), resources: demand});

	while needed.len() > 0 {
		let item = needed.pop().unwrap();
		let entry = cookbook.get(&item.name).unwrap();

		if entry.output < item.resources {
			needed.push(Ingredients{name: item.name.to_string(), resources: item.resources - entry.output});
		} else {
			let resource = resources.entry(item.name.to_string()).or_insert(0);
			*resource += entry.output - item.resources;
		}

		for dep in &entry.input {
			if dep.name == "ORE" {
				ore += dep.resources;
				continue;
			}

			let resource = resources.entry(dep.name.to_string()).or_insert(0);

			if *resource < dep.resources {
				needed.push(Ingredients{name: dep.name.to_string(), resources: dep.resources - *resource});
				*resource = 0; 
			} else {
				*resource = *resource - dep.resources;
			}
		}
	}

	ore
}

fn main() {
	let input: Vec<String> = include_str!("input")
		.trim()
		.split('\n')
		.map(|k| k.to_string())
		.collect();

	let (mut wanted, cookbook) = parse_input(input);
	let mut ores = calculate(&cookbook, 1);
	println!("Silver: {}", ores);
}
