use std::collections::VecDeque;

use super::intcode::*;

fn run_program(p: &Program, input: Option<&Vec<i64>>) -> VecDeque<i64> {
    let mut computer = Intcode::new(p.clone(), input);

    while !computer.is_halted() {
        computer.cycle();
    }

    computer.output
}

pub fn run(input_str: &String) {
    println!("\n-- Day 9 --");

    let input: Program = input_str
        .trim_end_matches('\n')
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    //==============================================================================================
    // Part 1
    let quine = vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99];
    assert_eq!(run_program(&quine, None), quine.into_iter().collect::<VecDeque<_>>());
    assert_eq!(run_program(&vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0], None).pop_front().unwrap().to_string().len(), 16);
    assert_eq!(run_program(&vec![104, 1125899906842624, 99], None).pop_front().unwrap(), 1125899906842624);

    println!("Part 1: {:?}", run_program(&input, Some(&vec![1])).pop_front().unwrap());

    //==============================================================================================
    // Part 2
    println!("Part 2: {:?}", run_program(&input, Some(&vec![2])).pop_front().unwrap());
}