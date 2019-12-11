use super::intcode::*;

fn calc_thrust(program: &Program, phase: &Vec<i32>, thrust: i32) -> i32 {
    let mut output = 0;

    let mut i: i32 = -1;
    let inputs = vec![phase[0], thrust];

    let mut inp = || -> Option<i32> {
        i = i + 1;
        Some(inputs[i as usize])
    };

    let mut outp = |x: i32| { output = x; };

    let mut c = Intcode::new(program.clone(), &mut inp, &mut outp);

    while !c.is_halted() {
        c.cycle();
    }

    if phase.len() > 1 {
        calc_thrust(program, &phase[1..].to_vec(), output)
    } else {
        output
    }
}

pub fn run(input_str: &String) {
    println!("\n-- Day 7 --");

    let input: Program = input_str
        .trim_end_matches('\n')
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Part 1
    println!("Part 1");
    assert_eq!(calc_thrust(&vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0], &vec![4, 3, 2, 1, 0], 0), 43210);
    assert_eq!(calc_thrust(&vec![3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0], &vec![0, 1, 2, 3, 4], 0), 54321);
    assert_eq!(calc_thrust(&vec![3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0], &vec![1, 0, 4, 3, 2], 0), 65210);

    let max = permutohedron::Heap::new(&mut vec![0, 1, 2, 3, 4])
        .map(|v| calc_thrust(&input, &v, 0))
        .max();

    println!("Part 1: {}", max.unwrap());
}
