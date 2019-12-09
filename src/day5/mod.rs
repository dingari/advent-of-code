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
        05 => (Op::CondJmp { cond: true, x: param(v[1], m1), dst: param(v[2], m2) }, 3),
        06 => (Op::CondJmp { cond: false, x: param(v[1], m1), dst: param(v[2], m2) }, 3),
        07 => (Op::CmpLess { x: param(v[1], m1), y: param(v[2], m2), dst: param(v[3], m3) }, 4),
        08 => (Op::CmpEq { x: param(v[1], m1), y: param(v[2], m2), dst: param(v[3], m3) }, 4),
        99 => (Op::Halt, 1),
        _ => panic!("Unknown opcode: {}", v[0]),
    }
}

fn run_program(input: &Program, input_fn: &dyn Fn() -> i32, output_fn: &dyn Fn(i32) -> ()) -> Program {
    let mut buf = input.clone();
    let mut pc: usize = 0;

    loop {
        let (op, num_increments) = parse_op(&buf[pc..buf.len()]);
        pc += num_increments;

        let read_val = |v: &Program, x: Param| -> i32 {
            match x {
                Param::Address { x } => v[x] as i32,
                Param::Immediate { x } => x,
            }
        };

        let write_val = |v: &mut Program, dst: Param, val: i32| {
            match dst {
                Param::Address { x } => v[x] = val,
                Param::Immediate { x: _ } => unreachable!(),
            };
        };

        let do_op = |v: &mut Program, x: Param, y: Param, dst: Param, op: &dyn Fn(i32, i32) -> i32| {
            write_val(v, dst, op(read_val(&v, x), read_val(&v, y)));
        };

        match op {
            Op::Add { x, y, dst } => do_op(&mut buf, x, y, dst, &|x, y| x + y),
            Op::Mul { x, y, dst } => do_op(&mut buf, x, y, dst, &|x, y| x * y),
            Op::Input { dst } => buf[dst] = input_fn(),
            Op::Output { out } => output_fn(buf[out]),
            Op::CondJmp { cond, x, dst } => if (read_val(&buf, x) > 0) == cond { pc = read_val(&buf, dst) as usize },
            Op::CmpLess { x, y, dst } => do_op(&mut buf, x, y, dst, &|x, y| if x < y { 1 } else { 0 }),
            Op::CmpEq { x, y, dst } => do_op(&mut buf, x, y, dst, &|x, y| if x == y { 1 } else { 0 }),
            Op::Halt => break,
        };
    }

    return buf;
}

pub fn run(input_str: &String) {
    println!("\n-- Day 5 --");

    let input: Program = input_str
        .trim_end_matches('\n')
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let print_output = |x: i32| { println!("{}", x); };

    // Part 1
    println!("Part 1");
    let get_input_1 = || -> i32 { 1 };
    assert_eq!(run_program(&vec![1002, 4, 3, 4, 33], &get_input_1, &print_output), vec![1002, 4, 3, 4, 99]);
    assert_eq!(run_program(&vec![1101, 100, -1, 4, 0], &get_input_1, &print_output), vec![1101, 100, -1, 4, 99]);

    run_program(&input, &get_input_1, &print_output);

    // Part 2
    let get_test_input = || -> i32 { 8 };
    let assert_output = |desired_out: i32| { move |out| assert_eq!(out, desired_out) };

    println!("\nPart 2");
    run_program(&vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &get_test_input, &assert_output(1));
    run_program(&vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &get_test_input, &assert_output(0));
    run_program(&vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], &get_test_input, &assert_output(1));
    run_program(&vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], &get_test_input, &assert_output(0));
    run_program(&vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], &get_test_input, &assert_output(1));
    run_program(&vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], &get_test_input, &assert_output(1));

    run_program(&input, &|| 5, &print_output);
}
