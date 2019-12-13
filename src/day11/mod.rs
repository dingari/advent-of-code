use std::collections::HashMap;

use itertools::{Itertools, MinMaxResult};
use num::complex::Complex;
use super::intcode::*;

type C = Complex<i32>;

fn painted_tiles(input: &Program, init_col: i64) -> HashMap<C, i64> {
    let mut robot = Intcode::new(input, None);
    let mut points = HashMap::<C, i64>::new();

    (0..1)
        .cycle()
        .try_fold((C::new(0, 0), C::new(0, 1)), |(pos, dir), _| {
            robot.input.push_back(*points.get(&pos).unwrap_or(&init_col));

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

    points
}

pub fn run(input_str: &str) {
    println!("\n-- Day 11 --");

    let input = super::parse_intcode_program(input_str);

    //==============================================================================================
    // Part 1
    let num_painted_tiles = painted_tiles(&input, 0).keys().len();
    assert_eq!(2226, num_painted_tiles);
    println!("Part 1: {}", num_painted_tiles);

    //==============================================================================================
    // Part 2
    println!("Part 2:");
    let tiles = painted_tiles(&input, 1);

    let (min_x, max_x) = match tiles.iter().map(|(&c, _)| c.re).minmax() {
        MinMaxResult::MinMax(min, max) => (min, max),
        _ => unreachable!(),
    };

    let (min_y, max_y) = match tiles.iter().map(|(&c, _)| c.im).minmax() {
        MinMaxResult::MinMax(min, max) => (min, max),
        _ => unreachable!(),
    };

    let mut disp = vec![vec![' '; (max_x - min_x) as usize + 1]; (max_y - min_y) as usize + 1];

    tiles.iter().for_each(|(&k, &v)| disp[(k.im - min_y) as usize][(k.re - min_x) as usize] = if v == 1 { 'x' } else { ' ' });
    disp.iter().rev().map(|l| l.iter().collect::<String>()).for_each(|l| println!("{}", l))
}
