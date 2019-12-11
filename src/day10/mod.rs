use std::collections::{HashSet, HashMap};

use num::Complex;
use itertools::Itertools;

//type P = (i32, i32);
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

fn visible_asteroids(asteroids: &HashSet<P>) -> HashMap<P, HashSet<P>> {
    let to_float = |c: Complex<i32>| -> Complex<f32> {
        Complex::<f32>::new(c.re as f32, c.im as f32)
    };

    asteroids
        .iter()
        .map(|&c| (c, asteroids
            .difference(&vec![c]
                .into_iter()
                .collect::<HashSet<P>>()
            )
            .map(|&p| (p, to_float(p - c).arg()))
            .sorted_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .group_by(|&x| x.1)
            .into_iter()
            .map(|(_, r)| r.collect::<Vec<_>>()
                .into_iter()
                .min_by(|a, b| (a.0 - c).norm_sqr().cmp(&(b.0 - c).norm_sqr()))
            )
            .filter(|o| o.is_some())
            .map(|o| o.unwrap().0)
            .collect::<HashSet<_>>()
        ))
        .collect::<HashMap<P, HashSet<P>>>()
}

fn max_visibility(asteroids: &HashSet<P>) -> Option<(P, usize)> {
    match visible_asteroids(asteroids).iter().max_by(|&v1, &v2| v1.1.len().cmp(&v2.1.len())) {
        Some((&k, v)) => Some((k, v.len())),
        None => None
    }
}

pub fn run(input_str: &String) {
    assert_eq!((P::new(3, 4), 8), max_visibility(&parse(".#..#\n.....\n#####\n....#\n...##")).unwrap());
    assert_eq!((P::new(5, 8), 33), max_visibility(&parse("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####")).unwrap());
    assert_eq!((P::new(1, 2), 35), max_visibility(&parse("#.#...#.#.\n.###....#.\n.#....#...\n##.#.#.#.#\n....#.#.#.\n.##..###.#\n..#...##..\n..##....##\n......#...\n.####.###.")).unwrap());
    assert_eq!((P::new(6, 3), 41), max_visibility(&parse(".#..#..###\n####.###.#\n....###.#.\n..###.##.#\n##.##.#.#.\n....###..#\n..#.#..#.#\n#..#.#.###\n.##...##.#\n.....#.#..")).unwrap());
    assert_eq!((P::new(11, 13), 210), max_visibility(&parse(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##")).unwrap());

    let max = max_visibility(&parse(&input_str)).unwrap();
    println!("Max: {:?}, len: {}", max.0, max.1);
}
