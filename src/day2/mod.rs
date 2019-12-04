use itertools::Itertools;

enum Op {
    Add { x: usize, y: usize, dst: usize },
    Mul { x: usize, y: usize, dst: usize },
    Halt,
}

fn run_program(input: &Vec<usize>) -> Vec<usize> {
    let parse_op = |v: &[usize]| -> Op {
        match v[0] {
            1 => Op::Add { x: v[1], y: v[2], dst: v[3] },
            2 => Op::Mul { x: v[1], y: v[2], dst: v[3] },
            99 => Op::Halt,
            _ => panic!("Unknown opcode: {}", v[0]),
        }
    };

    let mut out = input.clone();

    for i in (0..out.len()).step_by(4) {
        let end = if i + 4 >= out.len() { out.len() } else { i + 4 };
        let op = parse_op(&out[i..end]);

        match op {
            Op::Add { x, y, dst } => out[dst] = out[x] + out[y],
            Op::Mul { x, y, dst } => out[dst] = out[x] * out[y],
            Op::Halt => break,
        }
    }

    return out;
}

fn find_input(input: &Vec<usize>, desired_output: usize) -> (usize, usize) {
    let mut v = input.clone();

    for (i, j) in (1..100_usize).cartesian_product(1..100_usize) {
            v[1..=2].copy_from_slice(&[i, j]);

            let out = run_program(&v);
            if out[0] == desired_output {
                return (i, j);
            }
    }

    return (0, 0);
}

pub fn run(input_str: &String) {
    println!("\n-- Day 2 --");

    let mut input: Vec<usize> = input_str
        .trim_end_matches('\n')
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    // Part 1
    assert_eq!(run_program(&vec![1, 0, 0, 0, 99]), vec![2, 0, 0, 0, 99]);
    assert_eq!(run_program(&vec![2, 3, 0, 3, 99]), vec![2, 3, 0, 6, 99]);
    assert_eq!(run_program(&vec![2, 4, 4, 5, 99, 0]), vec![2, 4, 4, 5, 99, 9801]);
    assert_eq!(run_program(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]), vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);

    input[1..=2].copy_from_slice(&[12, 02]);
    println!("Part 1: {:?}", run_program(&input)[0]);

    // Part 2
    println!("Part 2: {:?}", find_input(&input, 19690720));
}