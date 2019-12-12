extern crate regex;

use num::Integer;
use regex::Regex;
use std::thread;

#[derive(Copy, Clone)]
struct System {
	coordinate: i64,
	index: usize,
	velocity: i64,
}

fn run_simulator(mut map: Vec<System>, steps: usize, index: usize) -> (Vec<System>, usize) {
	for _ in 0..steps {
		let filterable = map.clone();
		for i in 0..map.len() {
			let less: i64 = filterable
				.iter()
				.filter(|&x| x.coordinate < filterable[i].coordinate)
				.count() as i64;
			let more: i64 = filterable
				.iter()
				.filter(|&x| x.coordinate > filterable[i].coordinate)
				.count() as i64;

			map[i].velocity += more - less;
			map[i].coordinate += map[i].velocity;
		}
	}
	(map, index)
}

fn run_until_same(mut map: Vec<System>, index: usize) -> (u64, usize) {
	let mut step: u64 = 0;
	let mut coordinates = Vec::new();

	for i in 0..map.len() {
		coordinates.push(map[i].coordinate);
	}

	loop {
		step += 1;

		let filterable = map.clone();
		for i in 0..map.len() {
			let less: i64 = filterable
				.iter()
				.filter(|&x| x.coordinate < filterable[i].coordinate)
				.count() as i64;
			let more: i64 = filterable
				.iter()
				.filter(|&x| x.coordinate > filterable[i].coordinate)
				.count() as i64;

			map[i].velocity += more - less;
			map[i].coordinate += map[i].velocity;
		}

		for k in 0..map.len() {
			if map[k].coordinate != coordinates[k] {
				break;
			}

			let same = map.iter().filter(|&x| x.velocity == 0).count();
			if same == map.len() {
				return (step, index);
			}
		}
	}
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

fn part_one(solar_x: Vec<System>, solar_y: Vec<System>, solar_z: Vec<System>) -> i64 {
	let mut threads = Vec::new();
	let length = solar_x.len();
	let steps = 1000;

	threads.push(thread::spawn(move || -> (Vec<System>, usize) {
		run_simulator(solar_x, steps, 0)
	}));

	threads.push(thread::spawn(move || -> (Vec<System>, usize) {
		run_simulator(solar_y, steps, 1)
	}));

	threads.push(thread::spawn(move || -> (Vec<System>, usize) {
		run_simulator(solar_z, steps, 2)
	}));

	let mut potential = Vec::new();
	let mut kinetic = Vec::new();

	for _ in 0..length {
		potential.push(0);
		kinetic.push(0);
	}

	for i in threads {
		let solar = i.join().unwrap();
		for x in solar.0 {
			potential[x.index] += x.coordinate.abs();
			kinetic[x.index] += x.velocity.abs();
		}
	}

	let mut result = 0;

	for i in 0..length {
		result += potential[i] * kinetic[i];
	}

	result
}

fn part_two(solar_x: Vec<System>, solar_y: Vec<System>, solar_z: Vec<System>) -> u64 {
	let mut threads = Vec::new();
	threads.push(thread::spawn(move || -> (u64, usize) {
		run_until_same(solar_x, 0)
	}));

	threads.push(thread::spawn(move || -> (u64, usize) {
		run_until_same(solar_y, 1)
	}));

	threads.push(thread::spawn(move || -> (u64, usize) {
		run_until_same(solar_z, 2)
	}));

	let mut count = vec![0, 0, 0];

	for i in threads {
		let result = i.join().unwrap();
		count[result.1] = result.0;
	}

	count[0].lcm(&count[1]).lcm(&count[2])
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

	println!(
		"Silver: {}",
		part_one(solar_x.clone(), solar_y.clone(), solar_z.clone())
	);
	println!(
		"Gold: {}",
		part_two(solar_x.clone(), solar_y.clone(), solar_z.clone())
	);
}
