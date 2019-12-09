use itertools::Itertools;

use super::intcode::*;

fn run_program(input: &Program) -> Program {
    let mut computer = Intcode::new(input.clone(), None, None);

    while !computer.is_halted() {
        computer.cycle();
    }

    computer.program()
}

fn find_input(input: &Program, desired_output: i32) -> (i32, i32) {
    let mut v = input.clone();

    for (i, j) in (1..100_i32).cartesian_product(1..100_i32) {
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

    let mut input: Vec<i32> = input_str
        .trim_end_matches('\n')
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
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