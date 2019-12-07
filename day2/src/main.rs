enum OpCodes {
	Add = 1,
	Mul,
	Halt = 99,
}

impl OpCodes {
	fn from(code: i64) -> OpCodes {
		match code {
			1 => OpCodes::Add,
			2 => OpCodes::Mul,
			_ => OpCodes::Halt,
		}
	}

	fn param_count(&self) -> usize {
		match self {
			OpCodes::Add | OpCodes::Mul => 4,
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
}

impl OpMachine {
	fn get_triple(&mut self) -> (i64, i64, usize) {
		let verb = self.program[self.program[self.ip + 1] as usize];
		let noun = self.program[self.program[self.ip + 2] as usize];
		let loc = self.program[self.ip + 3] as usize;

		(verb, noun, loc)
	}

	fn add(&mut self) {
		let (verb, noun, loc) = self.get_triple();
		self.program[loc] = verb + noun;
	}

	fn mul(&mut self) {
		let (verb, noun, loc) = self.get_triple();
		self.program[loc] = verb * noun;
	}

	fn step(&mut self) -> bool {
		let code = OpCodes::from(self.program[self.ip]);

		match code {
			OpCodes::Add => self.add(),
			OpCodes::Mul => self.mul(),
			OpCodes::Halt => return false,
		}

		self.ip += code.param_count();
		true
	}
}

/**
 * Originally I calculated it on paper, but I still
 * wanted a function that handled it. So, beautiful
 * brute force it is!
 */
fn brute_force(mut program: Vec<i64>) -> i64 {
	/* desired number */
	let desired = 19690720;

	for noun in 1..100 {
		program[1] = noun;
		for verb in 1..100 {
			program[2] = verb;

			let mut machine = OpMachine {
				ip: 0,
				program: program.clone(),
			};
			while machine.step() {}

			if machine.program[0] == desired {
				return 100 * noun + verb;
			}
		}
	}

	return 0;
}

fn main() {
	let mut program: Vec<i64> = include_str!("input")
		.trim()
		.split(',')
		.map(|num| num.parse::<i64>().unwrap())
		.collect();

	/* this are requirements */
	program[1] = 12;
	program[2] = 2;

	let mut machine = OpMachine {
		ip: 0,
		program: program.clone(),
	};
	/* step over the code */
	while machine.step() {}
	println!("Silver {}", machine.program[0]);
	println!("Gold {}", brute_force(program.clone()));
}
