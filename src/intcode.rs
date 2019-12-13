use std::collections::VecDeque;

pub type Program = Vec<i64>;

#[derive(Debug, Copy, Clone)]
enum Param {
    Address { x: usize },
    Immediate { x: i64 },
    Relative { x: i64 },
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

    pub input: VecDeque<i64>,
    pub output: VecDeque<i64>,

    relative_base: usize,
    memory: Vec<i64>,
    program_size: usize,
}

impl Intcode {
    pub fn new(program: &Vec<i64>, init_input: Option<&[i64]>) -> Self {
        let mut v = vec![0; 2048];
        v[0..program.len()].copy_from_slice(program);

        Intcode {
            program: v,
            pc: 0,
            is_halted: false,
            input: match init_input {
                Some(v) => v.iter().cloned().collect(),
                None => vec![].into_iter().collect(),
            },
            output: VecDeque::<i64>::new(),
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

        let param = |x: i64, mode: usize| -> Param {
            match mode {
                0 => Param::Address { x: x as usize },
                1 => Param::Immediate { x: x as i64 },
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
            Op::AdjRelBase { x } => self.relative_base = (self.relative_base as i64 + self.read(x)) as usize,
            Op::Halt => self.is_halted = true,
        };
    }

    pub fn run_til_halt(&mut self) -> Option<i64> {
        while !self.is_halted {
            self.cycle();
        }

        self.output.pop_front()
    }

    pub fn run_til_output(&mut self) -> Option<i64> {
        match self.run_til_num_output(1) {
            Some(mut o) => o.pop_front(),
            None => None,
        }
    }

    pub fn run_til_num_output(&mut self, num: usize) -> Option<VecDeque<i64>> {
        let start = self.output.len();

        while self.output.len() < start + num && !self.is_halted {
            self.cycle();
        }

        match (self.is_halted, self.output.len() == start + num) {
            (_, true) => Some(self.output.drain(0..num).collect::<VecDeque<_>>()),
            (true, false) => None,
            _ => unreachable!(),
        }
    }

    fn perform_op(&mut self, x: Param, y: Param, dst: Param, op: fn(i64, i64) -> i64) {
        self.write(dst, op(self.read(x), self.read(y)));
    }

    fn read(&self, x: Param) -> i64 {
        match x {
            Param::Address { x } => self.program[x],
            Param::Immediate { x } => x,
            Param::Relative { x } => self.program[(self.relative_base as i64 + x) as usize],
        }
    }

    fn write(&mut self, dst: Param, val: i64) {
        match dst {
            Param::Address { x } => self.program[x] = val,
            Param::Immediate { x: _ } => unreachable!(),
            Param::Relative { x } => self.program[(self.relative_base as i64 + x) as usize] = val,
        };
    }
}
