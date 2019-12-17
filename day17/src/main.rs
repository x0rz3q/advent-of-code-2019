use std::collections::HashSet;
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
		if self.inputs.len() == 0 {
			println!("Need more");
		}
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

fn index(x: i64, y: i64) -> String {
	return format!("{}_{}", x, y);
}

fn silver(program: Vec<i64>) -> i64 {
	let mut machine = OpMachine {
		program: program.clone(),
		inputs: VecDeque::new(),
		outputs: VecDeque::new(),
		base: 0,
		ip: 0,
	};

	let mut map: HashSet<String> = HashSet::new();
	let mut x = 0;
	let mut y = 0;
	let mut max_x = 0;

	loop {
		let output = machine.run_until_output();
		if !output.is_some() {
			break;
		}

		match output.unwrap() {
			10 => {
				y += 1;

				if max_x < x {
					max_x = x;
				}

				x = 0;
			}
			35 => {
				map.insert(index(x, y));
				x += 1;
			}
			_ => {
				x += 1;
			}
		}
	}

	let mut output = 0;
	for i in 0..y + 1 {
		for j in 0..max_x + 1 {
			let mut valid = true;
			valid = valid && map.contains(&index(j - 1, i));
			valid = valid && map.contains(&index(j + 1, i));
			valid = valid && map.contains(&index(j, i + 1));
			valid = valid && map.contains(&index(j, i - 1));

			if valid {
				output += i * j;
			}
		}
	}

	output
}

/**
 * Reversed it by hand on paper.
 * A: R,8,L,12,R,4,R,4
 * B: R,8,L,10,L,12,R,4
 * C: R,8,L,10,R,8
 * Path: B,A,B,C,B,A,C,A,C,A
 */
fn gold(program: Vec<i64>) -> i64 {
	let path = "B,A,B,C,B,A,C,A,C,A\n";
	let sequence_a = "R,8,L,12,R,4,R,4\n";
	let sequence_b = "R,8,L,10,L,12,R,4\n";
	let sequence_c = "R,8,L,10,R,8\n";

	let mut program = program.clone();
	program[0] = 2;

	let mut machine = OpMachine {
		program: program,
		inputs: VecDeque::new(),
		outputs: VecDeque::new(),
		base: 0,
		ip: 0,
	};

	for i in path.chars() {
		machine.register_input(i as i64);
	}

	for i in sequence_a.chars() {
		machine.register_input(i as i64);
	}

	for i in sequence_b.chars() {
		machine.register_input(i as i64);
	}

	for i in sequence_c.chars() {
		machine.register_input(i as i64);
	}

	/* register no */
	machine.register_input('n' as i64);
	machine.register_input(10);

	loop {
		let output = machine.run_until_output().unwrap();
		if output > 255 {
			return output;
		}
	}
}

fn main() {
	let mut program: Vec<i64> = include_str!("input")
		.trim()
		.split(',')
		.map(|num| num.parse::<i64>().unwrap())
		.collect();

	for _ in 0..5000 {
		program.push(0);
	}

	let output = silver(program.clone());
	println!("Silver: {}", output);
	let output = gold(program.clone());
	println!("Gold: {}", output);
}
