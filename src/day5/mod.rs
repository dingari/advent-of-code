use itertools::Itertools;

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
    Halt,
}

type Program = Vec<i32>;

fn parse_op(v: &[i32]) -> (Op, usize) {
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

    let (op, m1, m2, m3) = split(v[0] as usize);

    match op {
        01 => (Op::Add { x: param(v[1], m1), y: param(v[2], m2), dst: param(v[3], m3) }, 4),
        02 => (Op::Mul { x: param(v[1], m1), y: param(v[2], m2), dst: param(v[3], m3) }, 4),
        03 => (Op::Input { dst: v[1] as usize }, 2),
        04 => (Op::Output { out: v[1] as usize }, 2),
        99 => (Op::Halt, 1),
        _ => panic!("Unknown opcode: {}", v[0]),
    }
}

fn run_program(input: &Program) -> Program {
    let mut buf = input.clone();
    let mut pc: usize = 0;

    loop {
        let (op, num_increments) = parse_op(&buf[pc..buf.len()]);

        let do_op = |v: &mut Program, x: Param, y: Param, dst: Param, op: &dyn Fn(i32, i32) -> i32| {
            let read_val = |v: &Program, x: Param| -> i32 {
                match x {
                    Param::Address { x } => v[x] as i32,
                    Param::Immediate { x } => x,
                }
            };

            let write_val = |v: &mut Program, dst: Param, val: i32| {
                match dst {
                    Param::Address { x } => v[x] = val,
                    Param::Immediate { x } => unreachable!(),
                };
            };

            write_val(v, dst, op(read_val(&v, x), read_val(&v, y)));
        };

        println!("Op: {:?}", op);

        match op {
            Op::Add { x, y, dst } => do_op(&mut buf, x, y, dst, &|x, y| x + y),
            Op::Mul { x, y, dst } => do_op(&mut buf, x, y, dst, &|x, y| x * y),
            Op::Input { dst } => buf[dst] = 1,
            Op::Output { out } => println!("{}", buf[out]),
            Op::Halt => break,
        };

        pc += num_increments;
    }

    return buf;
}

pub fn run(input_str: &String) {
    println!("\n-- Day 5 --");

    let mut input: Program = input_str
        .trim_end_matches('\n')
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    assert_eq!(run_program(&vec![1002, 4, 3, 4, 33]), vec![1002, 4, 3, 4, 99]);
    assert_eq!(run_program(&vec![1101, 100, -1, 4, 0]), vec![1101, 100, -1, 4, 99]);

    run_program(&input);
}
