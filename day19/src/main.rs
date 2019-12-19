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

fn in_beam(program: Vec<i64>, x: i64, y: i64) -> bool {
	let mut machine = OpMachine {
		ip: 0,
		program: program.clone(),
		inputs: VecDeque::new(),
		outputs: VecDeque::new(),
		base: 0,
	};

	machine.register_input(x);
	machine.register_input(y);

	let output = machine.run_until_output().unwrap();
	output == 1
}

/**
 * Simple loop and count to get required beam.
 */
fn silver(program: Vec<i64>) -> i64 {
	let mut beam = 0;

	for y in 0..50 {
		for x in 0..50 {
			if in_beam(program.clone(), x, y) {
				beam += 1;
			}
		}
	}

	beam
}

/**
 * Check if the coordinate point has the required width.
 * Assumption: x,y is inside the beam.
 */
fn has_width(program: Vec<i64>, x: i64, y: i64) -> bool {
	in_beam(program.clone(), x + 99, y)
}

/**
 * Check if a given point forms a square.
 * Assumption: x + 99, y has already been checked.
 */
fn has_square(program: Vec<i64>, x: i64, y: i64) -> bool {
	return in_beam(program.clone(), x, y + 99) && in_beam(program.clone(), x + 99, y + 99);
}

/**
 * Simple bruteforce solution, runs fast enough to solve this problem.
 * We can also solve this with algebra, and we could probably also optimize the bruteforce.
 * But it runs pretty fast now.
 */
fn gold(program: Vec<i64>) -> i64 {
	let mut y = 50;
	let mut x = 0;

	loop {
		/* find start point of beam */
		let mut machine = OpMachine {
			ip: 0,
			program: program.clone(),
			inputs: VecDeque::new(),
			outputs: VecDeque::new(),
			base: 0,
		};

		machine.register_input(x);
		machine.register_input(y);

		let output = machine.run_until_output().unwrap();
		/* not start of beam */
		if output == 0 {
			x += 1;
			continue;
		}

		if !has_width(program.clone(), x, y) {
			y += 1;
			x = x / 2;
			continue;
		}

		if has_square(program.clone(), x, y) {
			return x * 10000 + y;
		} else {
			x = x + 1;
		}
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

	println!("Silver: {}", silver(program.clone()));
	println!("Gold: {}", gold(program.clone()));
}
