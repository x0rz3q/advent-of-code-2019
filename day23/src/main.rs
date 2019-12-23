use std::collections::HashMap;
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
#[derive(Clone)]
struct OpMachine {
	ip: usize,
	program: Vec<i64>,
	inputs: VecDeque<i64>,
	outputs: VecDeque<i64>,
	base: i64,
	exit_code: usize,
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
			self.exit_code = 2;
			return;
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
		self.exit_code = 0;

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

		if self.exit_code > 0 {
			return false;
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

fn setup_machines(program: Vec<i64>) -> Vec<OpMachine> {
	let mut machines: Vec<OpMachine> = Vec::new();

	for i in 0..50 {
		let mut machine = OpMachine {
			program: program.clone(),
			inputs: VecDeque::new(),
			outputs: VecDeque::new(),
			base: 0,
			ip: 0,
			exit_code: 0,
		};

		machine.register_input(i);
		machines.push(machine);
	}

	machines
}

fn silver(program: Vec<i64>) -> i64 {
	/* history */
	let mut machines: Vec<OpMachine> = setup_machines(program);

	loop {
		for i in 0..50 {
			if machines[i].inputs.len() == 0 {
				machines[i].register_input(-1);
			}

			let to = machines[i].run_until_output();
			if machines[i].exit_code > 0 {
				continue;
			}
			let to = to.unwrap() as usize;
			let x = machines[i].run_until_output().unwrap();
			let y = machines[i].run_until_output().unwrap();

			if to == 255 {
				return y;
			}

			machines[to].register_input(x);
			machines[to].register_input(y);
		}
	}
}

fn gold(program: Vec<i64>) -> i64 {
	/* history */
	let mut machines: Vec<OpMachine> = setup_machines(program);

	let mut nat_x = 0;
	let mut nat_y = 0;
	let mut history: HashSet<(i64, i64)> = HashSet::new();

	machines[0].register_input(-1);
	loop {
		for i in 0..50 {
			let count = machines.iter().filter(|x| x.inputs.len() == 0).count();

			if count == 50 {
				machines[0].register_input(nat_x);
				machines[0].register_input(nat_y);

				if history.contains(&(nat_x, nat_y)) {
					return nat_y;
				} else {
					history.insert((nat_x, nat_y));
				}
				break;
			}

			if machines[i].inputs.len() == 0 {
				machines[i].register_input(-1);
			}

			let to = machines[i].run_until_output();

			if machines[i].exit_code > 0 {
				continue;
			}

			let to = to.unwrap() as usize;
			let x = machines[i].run_until_output().unwrap();
			let y = machines[i].run_until_output().unwrap();

			if to == 255 {
				nat_x = x;
				nat_y = y;
				continue;
			}

			machines[to].register_input(x);
			machines[to].register_input(y);
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
