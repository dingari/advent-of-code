use std::collections::{HashSet, HashMap};

use num::Complex;
use itertools::Itertools;

type P = (i32, i32);

fn parse(input: &str) -> HashSet<P> {
    input
        .trim_end_matches('\n')
        .split('\n')
        .enumerate()
        .map(|(y, s)| s
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .map(|(x, _)| (x as i32, y as i32))
            .collect::<Vec<P>>()
        )
        .flatten()
        .collect::<HashSet<P>>()
}

fn max_visibility(asteroids: &HashSet<P>) -> Option<(P, usize)> {
    let dist = |a: P, b: P| -> f64 {
        ((b.0 - a.0).pow(2) as f64 + (b.1 - a.1).pow(2) as f64).sqrt()
    };

    let v = asteroids
        .iter()
        .map(|&c| (c, asteroids
            .difference(&vec![c]
                .into_iter()
                .collect::<HashSet<P>>()
            )
            .map(|&t| (t.0 - c.0, t.1 - c.1))
            .map(|p| (p, Complex::<_>::new(p.0 as f32, p.1 as f32).arg()))
            .sorted_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(p, a)| ((p.0 + c.0, p.1 + c.1), a))
            .group_by(|&x| x.1)
            .into_iter()
            .map(|(_, r)| r.collect::<Vec<_>>()
                .into_iter()
                .min_by(|a, b| dist(c, a.0).partial_cmp(&dist(c, b.0)).unwrap())
            )
            .filter(|o| o.is_some())
            .map(|o| o.unwrap().0)
            .collect::<HashSet<_>>()
        ))
        .collect::<HashMap<P, HashSet<P>>>();

    match v.iter().max_by(|&v1, &v2| v1.1.len().cmp(&v2.1.len())) {
        Some((&k, v)) => Some((k, v.len())),
        None => None
    }
}

pub fn run(input_str: &String) {
    assert_eq!(((3, 4), 8), max_visibility(&parse(".#..#\n.....\n#####\n....#\n...##")).unwrap());
    assert_eq!(((5, 8), 33), max_visibility(&parse("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####")).unwrap());
    assert_eq!(((1, 2), 35), max_visibility(&parse("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.")).unwrap());
    assert_eq!(((6, 3), 41), max_visibility(&parse(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..")).unwrap());
    assert_eq!(((11, 13), 210), max_visibility(&parse(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##")).unwrap());

    let max = max_visibility(&parse(&input_str)).unwrap();
    println!("Max: {:?}, len: {}", max.0, max.1);
}
