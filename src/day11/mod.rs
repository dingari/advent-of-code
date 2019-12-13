use std::collections::HashMap;

use num::complex::Complex;
use super::intcode::*;

type C = Complex<i32>;

pub fn run(input_str: &str) {
    println!("\n-- Day 11 --");

    let input = super::parse_intcode_program(input_str);

    //==============================================================================================
    // Part 1
    let mut robot = Intcode::new(&input, None);
    let mut points = HashMap::<C, i64>::new();

    (0..1)
        .cycle()
        .try_fold((C::new(0, 0), C::new(0, 1)), |(pos, dir), _| {
            robot.input.push_back(*points.get(&pos).unwrap_or(&0));

            match robot.run_til_num_output(2) {
                Some(mut o) => {
                    points.insert(pos, o.pop_front().unwrap());

                    let new_dir = dir * match o.pop_front().unwrap() {
                        0 => C::new(0, 1),
                        1 => C::new(0, -1),
                        _ => unreachable!(),
                    };

                    Some((pos + new_dir, new_dir))
                }
                None => None
            }
        });

    assert_eq!(2226, points.keys().len());
    println!("Part 1: {}", points.keys().len());
}