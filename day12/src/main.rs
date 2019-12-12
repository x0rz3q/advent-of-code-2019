use std::collections::HashMap;
extern crate regex;

use regex::Regex;
use std::cmp;
use std::thread;

#[derive(Copy, Clone)]
struct System {
	coordinate: i64,
	index: usize,
	velocity: i64,
}

fn run_simulator(mut map: Vec<System>, steps: usize) -> Vec<System> {
	let length: i64 = (map.len() - 1) as i64;

	for _ in 0..steps {
		map.sort_by(|a, b| a.coordinate.cmp(&b.coordinate));

		for i in 0..map.len() {
			let index = i as i64;
			map[i].velocity += length - 2 * index;
			map[i].coordinate += map[i].velocity;
		}
	}

	map
}

fn get_numbers(input: String) -> (i64, i64, i64) {
	let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

	for cap in re.captures_iter(&input) {
		let x = &cap[1].parse::<i64>().unwrap();
		let y = &cap[2].parse::<i64>().unwrap();
		let z = &cap[3].parse::<i64>().unwrap();

		return (*x, *y, *z);
	}

	return (0, 0, 0);
}

fn main() {
	let mut solar_x: Vec<System> = Vec::new();
	let mut solar_y: Vec<System> = Vec::new();
	let mut solar_z: Vec<System> = Vec::new();
	let input: Vec<String> = include_str!("input")
		.trim()
		.split('\n')
		.map(|line| line.to_string())
		.collect();

	let mut index = 0;
	for line in input {
		let (x, y, z) = get_numbers(line.to_string());
		solar_x.push(System {
			index: index,
			coordinate: x,
			velocity: 0,
		});
		solar_y.push(System {
			index: index,
			coordinate: y,
			velocity: 0,
		});
		solar_z.push(System {
			index: index,
			coordinate: z,
			velocity: 0,
		});

		index += 1;
	}

	let mut threads = Vec::new();
	let steps = 4;
	threads.push(thread::spawn(move || -> Vec<System> {
		run_simulator(solar_x, steps)
	}));

//	threads.push(thread::spawn(move || -> Vec<System> {
//		run_simulator(solar_y, steps)
//	}));
//
//	threads.push(thread::spawn(move || -> Vec<System> {
//		run_simulator(solar_z, steps)
//	}));

	for i in threads {
		let solar_x = i.join().unwrap();
		println!("{}: {}", solar_x[0].index, solar_x[0].velocity);
	}
}
