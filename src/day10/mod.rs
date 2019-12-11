use std::collections::{HashSet, HashMap};
use std::f32::consts::PI;
use std::cmp::Ordering::Equal;

use num::Complex;
use itertools::Itertools;

type P = Complex<i32>;

fn parse(input: &str) -> HashSet<P> {
    input
        .trim_end_matches('\n')
        .split('\n')
        .enumerate()
        .map(|(y, s)| s
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(x, _)| P::new(x as i32, y as i32))
            .collect::<Vec<P>>()
        )
        .flatten()
        .collect::<HashSet<P>>()
}

fn to_float(c: Complex<i32>) -> Complex<f32> {
    Complex::<f32>::new(c.re as f32, c.im as f32)
}

fn max_visibility(asteroids: &HashSet<P>) -> Option<(P, usize)> {
    asteroids
        .iter()
        .map(|&c| (c, asteroids
            .difference(&vec![c]
                .into_iter()
                .collect::<HashSet<P>>()
            )
            .map(|&p| to_float(p - c).arg())
            .sorted_by(|&a, &b| a.partial_cmp(&b).unwrap_or(Equal))
            .dedup()
            .count()
        ))
        .max_by(|&(_, a), &(_, b)| a.cmp(&b))
}

fn lazer(source: P, asteroids: &HashSet<P>, target: usize) -> Option<P> {
    let unwrap_phase = |p: f32| if p < 0.0 { p + 2.0 * PI } else { p };

    asteroids
        .difference(&vec![source]
            .into_iter()
            .collect::<HashSet<P>>()
        )
        .map(|&p| (p, unwrap_phase(to_float((p - source) * P::new(0, 1)).arg()), (p - source).norm_sqr()))
        .sorted_by(|&(_, a, _), &(_, b, _)| a.partial_cmp(&b).unwrap_or(Equal))
        .group_by(|&(_, ph, _)| ph)
        .into_iter()
        .map(|(_, r)| r.into_iter().sorted_by(|&(_, _, a), &(_, _, b)| a.cmp(&b)).enumerate())
        .kmerge_by(|&(i, a), &(j, b)| if i == j { a.1 < b.1 } else { i < j })
        .map(|(_, (p, _, _))| p)
        .nth(target - 1)
}

pub fn run(input_str: &String) {
    let test = vec![
        ".#..#\n.....\n#####\n....#\n...##",
        "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####",
        "#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.",
        ".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..",
        ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##",
    ];

    // Part 1
    assert_eq!((P::new(3, 4), 8), max_visibility(&parse(test[0])).unwrap());
    assert_eq!((P::new(5, 8), 33), max_visibility(&parse(test[1])).unwrap());
    assert_eq!((P::new(1, 2), 35), max_visibility(&parse(test[2])).unwrap());
    assert_eq!((P::new(6, 3), 41), max_visibility(&parse(test[3])).unwrap());
    assert_eq!((P::new(11, 13), 210), max_visibility(&parse(test[4])).unwrap());

    let (a, visible) = max_visibility(&parse(&input_str)).unwrap();
    println!("Part 1: ({}, {}), num visible: {}", a.re, a.im, visible);

    // Part 2
    assert_eq!(P::new(11, 12), lazer(P::new(11, 13), &parse(test[4]), 1).unwrap());
    assert_eq!(P::new(12, 1), lazer(P::new(11, 13), &parse(test[4]), 2).unwrap());
    assert_eq!(P::new(16, 0), lazer(P::new(11, 13), &parse(test[4]), 20).unwrap());
    assert_eq!(P::new(8, 2), lazer(P::new(11, 13), &parse(test[4]), 200).unwrap());

    let a = lazer(a, &parse(&input_str), 200).unwrap();
    println!("Part 2: ({}, {})", a.re, a.im);
}
