use super::intcode::*;

fn run_program(p: &Program, input: &Vec<i64>) -> (Program, Option<i64>) {
    let mut computer = Intcode::new(p.clone(), Some(input));

    while !computer.is_halted() {
        computer.cycle();
    }

    (computer.program(), computer.output.pop_front())
}

pub fn run(input_str: &String) {
    println!("\n-- Day 5 --");

    let input: Program = input_str
        .trim_end_matches('\n')
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    //==============================================================================================
    // Part 1
    assert_eq!(run_program(&vec![1002, 4, 3, 4, 33], &vec![1]).0, vec![1002, 4, 3, 4, 99]);
    assert_eq!(run_program(&vec![1101, 100, -1, 4, 0], &vec![1]).0, vec![1101, 100, -1, 4, 99]);

    run_program(&input, &vec![1]);
    println!("Part 1: {}", run_program(&input, &vec![1]).1.unwrap());

    //==============================================================================================
    // Part 2
    assert_eq!(Some(1), run_program(&vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8], &vec![8]).1);
    assert_eq!(Some(0), run_program(&vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8], &vec![8]).1);
    assert_eq!(Some(1), run_program(&vec![3, 3, 1108, -1, 8, 3, 4, 3, 99], &vec![8]).1);
    assert_eq!(Some(0), run_program(&vec![3, 3, 1107, -1, 8, 3, 4, 3, 99], &vec![8]).1);
    assert_eq!(Some(1), run_program(&vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9], &vec![8]).1);
    assert_eq!(Some(1), run_program(&vec![3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1], &vec![8]).1);

    println!("Part 2: {}", run_program(&input, &vec![5]).1.unwrap());
}
