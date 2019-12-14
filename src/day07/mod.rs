use super::intcode::*;

fn calc_thrust(program: &Program, phase: &Vec<i64>) -> i64 {
    phase.iter().fold(0, |acc, &p| {
        let mut c = Intcode::new(program, Some(&vec![p, acc]));

        c.run_til_output().unwrap()
    })
}

fn feedback(program: &Program, phase: &Vec<i64>) -> i64 {
    let mut amps = phase
        .iter()
        .map(|&p| Intcode::new(program, Some(&vec![p])))
        .collect::<Vec<_>>();

    let mut out = 0;
    while let Some(o) = amps
        .iter_mut()
        .fold(Some(out), |acc, c| {
            c.input.push_back(acc.unwrap_or(0));

            c.run_til_output()
        }) {
        out = o;
    }

    out
}

pub fn run(input_str: &str) {
    println!("\n-- Day 7 --");

    let input = super::parse_intcode_program(input_str);

    //==============================================================================================
    // Part 1
    assert_eq!(43210, calc_thrust(&vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0], &vec![4, 3, 2, 1, 0]));
    assert_eq!(54321, calc_thrust(&vec![3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0], &vec![0, 1, 2, 3, 4]));
    assert_eq!(65210, calc_thrust(&vec![3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0], &vec![1, 0, 4, 3, 2]));

    let max = permutohedron::Heap::new(&mut vec![0, 1, 2, 3, 4])
        .map(|v| calc_thrust(&input, &v))
        .max();

    println!("Part 1: {}", max.unwrap());

    //==============================================================================================
    // Part 2
    assert_eq!(139629729, feedback(&vec![3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5], &vec![9, 8, 7, 6, 5]));
    assert_eq!(18216, feedback(&vec![3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10], &vec![9, 7, 8, 5, 6]));

    let max = permutohedron::Heap::new(&mut vec![5, 6, 7, 8, 9])
        .map(|v| feedback(&input, &v))
        .max();

    println!("Part 2: {}", max.unwrap());
}
