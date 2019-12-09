pub type Program = Vec<i32>;
pub type Input = dyn Fn() -> i32;
pub type Output = dyn Fn(i32) -> ();

#[derive(Debug, Copy, Clone)]
enum Param {
    Address { x: usize },
    Immediate { x: i32 },
}

#[derive(Debug)]
enum Op {
    Add { x: Param, y: Param, dst: Param },
    Mul { x: Param, y: Param, dst: Param },
    Input { dst: usize },
    Output { out: usize },
    CondJmp { cond: bool, x: Param, dst: Param },
    CmpLess { x: Param, y: Param, dst: Param },
    CmpEq { x: Param, y: Param, dst: Param },
    Halt,
}

pub struct Intcode<'a> {
    program: Program,
    pc: usize,
    is_halted: bool,

    input_fn: Option<&'a Input>,
    output_fn: Option<&'a Output>,
}

impl<'a> Intcode<'a> {
    pub fn new(program: Vec<i32>, input_fn: Option<&'a Input>, output_fn: Option<&'a Output>) -> Self {
        Intcode {
            program,
            pc: 0,
            is_halted: false,
            input_fn,
            output_fn,
        }
    }

    pub fn cycle(&mut self) {
        let (op, num_increments) = self.fetch();
        self.pc += num_increments;

        self.execute(op);
    }

    pub fn is_halted(&self) -> bool {
        self.is_halted
    }

    pub fn program(&self) -> Program {
        self.program.clone()
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
                _ => unreachable!("{}, {}", x, mode),
            }
        };

        let v = &self.program[self.pc..];
        let (op, m1, m2, m3) = split(v[0] as usize);

        match op {
            01 => (Op::Add { x: param(v[1], m1), y: param(v[2], m2), dst: param(v[3], m3) }, 4),
            02 => (Op::Mul { x: param(v[1], m1), y: param(v[2], m2), dst: param(v[3], m3) }, 4),
            03 => (Op::Input { dst: v[1] as usize }, 2),
            04 => (Op::Output { out: v[1] as usize }, 2),
            05 => (Op::CondJmp { cond: true, x: param(v[1], m1), dst: param(v[2], m2) }, 3),
            06 => (Op::CondJmp { cond: false, x: param(v[1], m1), dst: param(v[2], m2) }, 3),
            07 => (Op::CmpLess { x: param(v[1], m1), y: param(v[2], m2), dst: param(v[3], m3) }, 4),
            08 => (Op::CmpEq { x: param(v[1], m1), y: param(v[2], m2), dst: param(v[3], m3) }, 4),
            99 => (Op::Halt, 1),
            _ => panic!("Unknown opcode: {}", v[0]),
        }
    }

    fn execute(&mut self, op: Op) {
        match op {
            Op::Add { x, y, dst } => self.perform_op(x, y, dst, |x, y| x + y),
            Op::Mul { x, y, dst } => self.perform_op(x, y, dst, |x, y| x * y),
            Op::Input { dst } => self.program[dst] = (self.input_fn.unwrap())(),
            Op::Output { out } => (self.output_fn.unwrap())(self.program[out]),
            Op::CondJmp { cond, x, dst } => if (self.read(x) > 0) == cond { self.pc = self.read(dst) as usize },
            Op::CmpLess { x, y, dst } => self.perform_op(x, y, dst, |x, y| if x < y { 1 } else { 0 }),
            Op::CmpEq { x, y, dst } => self.perform_op(x, y, dst, |x, y| if x == y { 1 } else { 0 }),
            Op::Halt => self.is_halted = true,
        };
    }

    fn perform_op(&mut self, x: Param, y: Param, dst: Param, op: fn(i32, i32) -> i32) {
        self.write(dst, op(self.read(x), self.read(y)));
    }

    fn read(&self, x: Param) -> i32 {
        match x {
            Param::Address { x } => self.program[x],
            Param::Immediate { x } => x,
        }
    }

    fn write(&mut self, dst: Param, val: i32) {
        match dst {
            Param::Address { x } => self.program[x] = val,
            Param::Immediate { x: _ } => unreachable!(),
        };
    }
}
