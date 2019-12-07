enum OpCodes {
    Add = 1,
    Mul,
    Input,
    Print,
    JIT,
    JIF,
    LT,
    EQ,
    Halt = 99,
}

enum OpModes {
    Parametric,
    Immediate,
}

impl OpCodes {
    fn from(code: i64) -> OpCodes {
        match code {
            1 => OpCodes::Add,
            2 => OpCodes::Mul,
            3 => OpCodes::Input,
            4 => OpCodes::Print,
            5 => OpCodes::JIT,
            6 => OpCodes::JIF,
            7 => OpCodes::LT,
            8 => OpCodes::EQ,
            _ => OpCodes::Halt,
        }
    }

    fn param_count(&self) -> usize {
        match self {
            OpCodes::Add | OpCodes::Mul | OpCodes::LT | OpCodes::EQ => 4,
            OpCodes::Input | OpCodes::Print => 2,
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
    fn get_variable(&self, mode: &OpModes, position: usize) -> i64 {
        match mode {
            OpModes::Immediate => self.program[position],
            _ => self.program[self.program[position] as usize],
        }
    }

    fn get_triple(&mut self, modes: Vec<OpModes>) -> (i64, i64, usize) {
        let verb = self.get_variable(&modes[0], self.ip + 1);
        let noun = self.get_variable(&modes[1], self.ip + 2);
        let loc = self.get_variable(&OpModes::Immediate, self.ip + 3) as usize;

        (verb, noun, loc)
    }

    fn get_double(&mut self, modes: Vec<OpModes>) -> (i64, i64) {
        let verb = self.get_variable(&modes[0], self.ip + 1);
        let noun = self.get_variable(&modes[1], self.ip + 2);

        (verb, noun)
    }

    fn add(&mut self, modes: Vec<OpModes>) {
        let (verb, noun, loc) = self.get_triple(modes);
        self.program[loc] = verb + noun;
    }

    fn mul(&mut self, modes: Vec<OpModes>) {
        let (verb, noun, loc) = self.get_triple(modes);
        self.program[loc] = verb * noun;
    }

    fn print(&mut self) {
        let verb = self.get_variable(&OpModes::Immediate, self.ip + 1) as usize;
        println!("{}", self.program[verb]);
    }

    fn input(&mut self) {
        let verb = self.get_variable(&OpModes::Immediate, self.ip + 1) as usize;
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).expect("Failed");
        self.program[verb] = buffer.trim().parse::<i64>().unwrap();
    }

    fn jump_if_true(&mut self, modes: Vec<OpModes>) {
        let (verb, noun) = self.get_double(modes);
        if noun < 0 {
            println!("Noun smaller than 0");
        }
        if verb != 0 {
            self.ip = noun as usize;
        } else {
            self.ip += 3;
        }
    }

    fn jump_if_false(&mut self, modes: Vec<OpModes>) {
        let (verb, noun) = self.get_double(modes);
        if noun < 0 {
            println!("Noun smaller than 0");
        }
        if verb == 0 {
            self.ip = noun as usize;
        } else {
            self.ip += 3;
        }
    }

    fn less_than(&mut self, modes: Vec<OpModes>) {
        let (verb, noun, loc) = self.get_triple(modes);
        if verb < noun {
            self.program[loc] = 1;
        } else {
            self.program[loc] = 0;
        }
    }

    fn equal(&mut self, modes: Vec<OpModes>) {
        let (verb, noun, loc) = self.get_triple(modes);
        if verb == noun {
            self.program[loc] = 1;
        } else {
            self.program[loc] = 0;
        }
    }

    fn get_mode(&self, input: i64) -> OpModes {
        if input % 10 == 1 {
            return OpModes::Immediate;
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
            OpCodes::Input => self.input(),
            OpCodes::Print => self.print(),
            OpCodes::JIT => self.jump_if_true(modes),
            OpCodes::JIF => self.jump_if_false(modes),
            OpCodes::LT => self.less_than(modes),
            OpCodes::EQ => self.equal(modes),
            OpCodes::Halt => return false,
        }

        self.ip += code.param_count();
        true
    }

    fn run(&mut self) {
        while self.step() {}
    }
}

fn main() {
    let program: Vec<i64> = include_str!("input")
        .trim()
        .split(',')
        .map(|num| num.parse::<i64>().unwrap())
        .collect();

    let mut machine = OpMachine {
        ip: 0,
        program: program.clone(),
    };

    /* step over the code */
    machine.run();
}
