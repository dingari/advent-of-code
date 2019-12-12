use std::collections::VecDeque;

pub type Program = Vec<i32>;

#[derive(Debug, Copy, Clone)]
enum Param {
    Address { x: usize },
    Immediate { x: i32 },
    Relative { x: i32 },
}

#[derive(Debug)]
enum Op {
    Add { x: Param, y: Param, dst: Param },
    Mul { x: Param, y: Param, dst: Param },
    Input { dst: Param },
    Output { out: Param },
    CondJmp { cond: bool, x: Param, dst: Param },
    CmpLess { x: Param, y: Param, dst: Param },
    CmpEq { x: Param, y: Param, dst: Param },
    AdjRelBase { x: Param },
    Halt,
}

pub struct Intcode {
    program: Program,
    pc: usize,
    is_halted: bool,

    pub input: VecDeque<i32>,
    pub output: VecDeque<i32>,

    relative_base: usize,
    memory: Vec<i32>,
    program_size: usize,
}

impl Intcode {
    pub fn new(program: Vec<i32>, init_input: Option<&Vec<i32>>) -> Self {
        let mut v = vec![0; 1024];
        v[0..program.len()].copy_from_slice(&program);

        Intcode {
            program: v,
            pc: 0,
            is_halted: false,
            input: match init_input {
                Some(v) => v.iter().cloned().collect(),
                None => vec![].into_iter().collect(),
            },
            output: VecDeque::<i32>::new(),
            relative_base: 0,
            memory: vec![0; 1024],
            program_size: program.len(),
        }
    }

    pub fn cycle(&mut self) {
        if !self.is_halted {
            let (op, num_increments) = self.fetch();
            self.pc += num_increments;

            self.execute(op);
        }
    }

    pub fn is_halted(&self) -> bool {
        self.is_halted
    }

    pub fn program(&self) -> Program {
        self.program[0..self.program_size].to_vec()
    }

    fn fetch(&self) -> (Op, usize) {
        let split = |x: usize| -> (usize, usize, usize, usize) {
            let get_digit = |x: usize, d: usize| -> usize {
                let p = 10_usize.pow(d as u32);
                (x % (10 * p)) / p
            };

            assert_eq!(get_digit(100, 2), 1);

            (x % 100, get_digit(x, 2), get_digit(x, 3), get_digit(x, 4))
        };

        let param = |x: i32, mode: usize| -> Param {
            match mode {
                0 => Param::Address { x: x as usize },
                1 => Param::Immediate { x: x as i32 },
                2 => Param::Relative { x },
                _ => unreachable!("{}, {}", x, mode),
            }
        };

        let v = &self.program[self.pc..];
        let (op, m1, m2, m3) = split(v[0] as usize);

        match op {
            01 => (Op::Add { x: param(v[1], m1), y: param(v[2], m2), dst: param(v[3], m3) }, 4),
            02 => (Op::Mul { x: param(v[1], m1), y: param(v[2], m2), dst: param(v[3], m3) }, 4),
            03 => (Op::Input { dst: param(v[1], m1) }, 2),
            04 => (Op::Output { out: param(v[1], m1) }, 2),
            05 => (Op::CondJmp { cond: true, x: param(v[1], m1), dst: param(v[2], m2) }, 3),
            06 => (Op::CondJmp { cond: false, x: param(v[1], m1), dst: param(v[2], m2) }, 3),
            07 => (Op::CmpLess { x: param(v[1], m1), y: param(v[2], m2), dst: param(v[3], m3) }, 4),
            08 => (Op::CmpEq { x: param(v[1], m1), y: param(v[2], m2), dst: param(v[3], m3) }, 4),
            09 => (Op::AdjRelBase { x: param(v[1], m1) }, 2),
            99 => (Op::Halt, 1),
            _ => panic!("Unknown opcode: {}", v[0]),
        }
    }

    fn execute(&mut self, op: Op) {
        match op {
            Op::Add { x, y, dst } => self.perform_op(x, y, dst, |x, y| x + y),
            Op::Mul { x, y, dst } => self.perform_op(x, y, dst, |x, y| x * y),
            Op::Input { dst } => match self.input.pop_front() {
                Some(x) => self.write(dst, x),
                None => panic!("No input provided"),
            },
            Op::Output { out } => self.output.push_back(self.read(out)),
            Op::CondJmp { cond, x, dst } => if (self.read(x) > 0) == cond { self.pc = self.read(dst) as usize },
            Op::CmpLess { x, y, dst } => self.perform_op(x, y, dst, |x, y| if x < y { 1 } else { 0 }),
            Op::CmpEq { x, y, dst } => self.perform_op(x, y, dst, |x, y| if x == y { 1 } else { 0 }),
            Op::AdjRelBase { x } => self.relative_base = self.read(x) as usize,
            Op::Halt => self.is_halted = true,
        };
    }

    pub fn run_til_halt(&mut self) -> Option<i32> {
        while !self.is_halted {
            self.cycle();
        }

        self.output.pop_front()
    }

    pub fn run_til_output(&mut self) -> Option<i32> {
        let num_out = self.output.len();

        while self.output.len() == num_out && !self.is_halted {
            self.cycle();
        }

        self.output.pop_front()
    }

    fn perform_op(&mut self, x: Param, y: Param, dst: Param, op: fn(i32, i32) -> i32) {
        self.write(dst, op(self.read(x), self.read(y)));
    }

    fn read(&self, x: Param) -> i32 {
        match x {
            Param::Address { x } => self.program[x],
            Param::Immediate { x } => x,
            Param::Relative { x } => self.program[(self.relative_base as i32 + x) as usize],
        }
    }

    fn write(&mut self, dst: Param, val: i32) {
        match dst {
            Param::Address { x } => self.program[x] = val,
            Param::Immediate { x: _ } => unreachable!(),
            Param::Relative { x } => self.program[(self.relative_base as i32 + x) as usize] = val,
        };
    }
}
