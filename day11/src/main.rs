use std::collections::HashMap;
use std::collections::VecDeque;

enum OpCodes {
	Add = 1,
	Mul,
	Input,
	Output,
	JIT,
	JIF,
	LT,
	EQ,
	RA,
	Halt = 99,
}

enum OpModes {
	Parametric,
	Immediate,
	Relative,
}

impl OpCodes {
	fn from(code: i64) -> OpCodes {
		match code {
			1 => OpCodes::Add,
			2 => OpCodes::Mul,
			3 => OpCodes::Input,
			4 => OpCodes::Output,
			5 => OpCodes::JIT,
			6 => OpCodes::JIF,
			7 => OpCodes::LT,
			8 => OpCodes::EQ,
			9 => OpCodes::RA,
			_ => OpCodes::Halt,
		}
	}

	fn param_count(&self) -> usize {
		match self {
			OpCodes::Add | OpCodes::Mul | OpCodes::LT | OpCodes::EQ => 4,
			OpCodes::Input | OpCodes::Output | OpCodes::RA => 2,
			_ => 0,
		}
	}
}

/**
 * OpMachine, it handles the operations step by step.
 * It uses the OpCodes struct above.
 */
struct OpMachine {
	ip: usize,
	program: Vec<i64>,
	inputs: VecDeque<i64>,
	outputs: VecDeque<i64>,
	base: i64,
}

impl OpMachine {
	fn get_value(&self, mode: &OpModes, position: usize) -> i64 {
		match mode {
			OpModes::Immediate => self.program[position],
			OpModes::Relative => self.program[(self.base + self.program[position]) as usize],
			_ => self.program[self.program[position] as usize],
		}
	}

	fn store(&mut self, value: i64, position: usize, mode: &OpModes) {
		let loc = match mode {
			OpModes::Relative => (self.base + self.program[position]) as usize,
			_ => self.program[position] as usize,
		};

		self.program[loc] = value;
	}

	fn add(&mut self, modes: Vec<OpModes>) {
		let verb = self.get_value(&modes[0], self.ip + 1);
		let noun = self.get_value(&modes[1], self.ip + 2);
		self.store(verb + noun, self.ip + 3, &modes[2]);
	}

	fn mul(&mut self, modes: Vec<OpModes>) {
		let verb = self.get_value(&modes[0], self.ip + 1);
		let noun = self.get_value(&modes[1], self.ip + 2);
		self.store(verb * noun, self.ip + 3, &modes[2]);
	}

	fn output(&mut self, mode: &OpModes) {
		let verb = self.get_value(mode, self.ip + 1);
		self.outputs.push_back(verb);
	}

	fn input(&mut self, mode: &OpModes) {
		let value = self.inputs.pop_front().unwrap();
		self.store(value, self.ip + 1, mode);
	}

	fn jump_if_true(&mut self, modes: Vec<OpModes>) {
		let verb = self.get_value(&modes[0], self.ip + 1);
		let noun = self.get_value(&modes[1], self.ip + 2);

		if verb != 0 {
			self.ip = noun as usize;
		} else {
			self.ip += 3;
		}
	}

	fn register_input(&mut self, input: i64) {
		self.inputs.push_back(input);
	}

	fn has_output(&self) -> bool {
		return !self.outputs.is_empty();
	}

	fn jump_if_false(&mut self, modes: Vec<OpModes>) {
		let verb = self.get_value(&modes[0], self.ip + 1);
		let noun = self.get_value(&modes[1], self.ip + 2);
		if verb == 0 {
			self.ip = noun as usize;
		} else {
			self.ip += 3;
		}
	}

	fn less_than(&mut self, modes: Vec<OpModes>) {
		let verb = self.get_value(&modes[0], self.ip + 1);
		let noun = self.get_value(&modes[1], self.ip + 2);

		if verb < noun {
			self.store(1, self.ip + 3, &modes[2]);
		} else {
			self.store(0, self.ip + 3, &modes[2]);
		}
	}

	fn equal(&mut self, modes: Vec<OpModes>) {
		let verb = self.get_value(&modes[0], self.ip + 1);
		let noun = self.get_value(&modes[1], self.ip + 2);
		if verb == noun {
			self.store(1, self.ip + 3, &modes[2]);
		} else {
			self.store(0, self.ip + 3, &modes[2]);
		}
	}

	fn relative_add(&mut self, modes: Vec<OpModes>) {
		let verb = self.get_value(&modes[0], self.ip + 1);
		self.base += verb;
	}

	fn get_mode(&self, input: i64) -> OpModes {
		if input % 10 == 1 {
			return OpModes::Immediate;
		} else if input % 10 == 2 {
			return OpModes::Relative;
		}

		OpModes::Parametric
	}

	fn step(&mut self) -> bool {
		let opcode = self.program[self.ip];
		let code = OpCodes::from(opcode % 100);
		let modes = vec![
			self.get_mode(opcode / 100),
			self.get_mode(opcode / 1000),
			self.get_mode(opcode / 10000),
		];

		match code {
			OpCodes::Add => self.add(modes),
			OpCodes::Mul => self.mul(modes),
			OpCodes::Input => self.input(&modes[0]),
			OpCodes::Output => self.output(&modes[0]),
			OpCodes::JIT => self.jump_if_true(modes),
			OpCodes::JIF => self.jump_if_false(modes),
			OpCodes::LT => self.less_than(modes),
			OpCodes::EQ => self.equal(modes),
			OpCodes::RA => self.relative_add(modes),
			OpCodes::Halt => return false,
		}

		self.ip += code.param_count();
		true
	}

	fn run_until_output(&mut self) -> Option<i64> {
		while self.step() {
			if self.has_output() {
				return Some(self.outputs.pop_front().unwrap());
			}
		}

		return None;
	}

	fn run(&mut self) {
		let mut result = self.run_until_output();
		while result.is_some() {
			println!("{}", result.unwrap());
			result = self.run_until_output();
		}
	}
}

enum Direction {
	Up,
	Left,
	Down,
	Right,
}

fn turn_left(dir: &Direction) -> Direction {
	match dir {
		Direction::Up => Direction::Left,
		Direction::Left => Direction::Down,
		Direction::Down => Direction::Right,
		Direction::Right => Direction::Up,
	}
}

fn turn_right(dir: &Direction) -> Direction {
	match dir {
		Direction::Left => Direction::Up,
		Direction::Down => Direction::Left,
		Direction::Right => Direction::Down,
		Direction::Up => Direction::Right,
	}
}

fn robot(mut machine: OpMachine) -> HashMap<String, i64> {
	let mut x = 5;
	let mut y = 0;
	/* 0 = up, 1 = left, 2 = down, 3 = right */
	let mut facing = Direction::Up;
	let mut map: HashMap<String, i64> = HashMap::new();

	loop {
		let color = machine.run_until_output();
		if !color.is_some() {
			break;
		}

		let color = color.unwrap();
		let index = format!("{}_{}", x, y);
		let point = map.entry(index.to_string()).or_insert(color);
		*point = color;

		let direction = machine.run_until_output().unwrap();
		if direction == 0 {
			facing = turn_left(&facing);
		} else {
			facing = turn_right(&facing);
		}

		match facing {
			Direction::Up => x += 1,
			Direction::Left => y -= 1,
			Direction::Right => y += 1,
			Direction::Down => x -= 1,
		};

		let input = match map.get(&format!("{}_{}", x, y)) {
			Some(point) => *point,
			None => 0,
		};

		machine.register_input(input);
	}

	map
}

fn part_one(program: Vec<i64>) -> usize {
	let mut machine = OpMachine {
		ip: 0,
		program: program.clone(),
		inputs: VecDeque::new(),
		outputs: VecDeque::new(),
		base: 0,
	};

	machine.register_input(0);
	let map = robot(machine);
	return map.len();
}

fn part_two(program: Vec<i64>) {
	let mut machine = OpMachine {
		ip: 0,
		program: program.clone(),
		inputs: VecDeque::new(),
		outputs: VecDeque::new(),
		base: 0,
	};

	machine.register_input(1);
	let map = robot(machine);

	let max_x = 5;
	for x in 0..6 {
		for y in 0..50 {
			let point = match map.get(&format!("{}_{}", max_x - x, y)) {
				Some(point) => *point,
				None => 0,
			};

			if point == 0 {
				print!(" ");
			} else {
				print!("▮");
			}
		}

		println!("");
	}
}

fn main() {
	let mut program: Vec<i64> = include_str!("input")
		.trim()
		.split(',')
		.map(|num| num.parse::<i64>().unwrap())
		.collect();

	for _ in 0..1000 {
		program.push(0);
	}

	println!("Silver: {}", part_one(program.clone()));
	println!("Gold: ");
	part_two(program.clone());
}
