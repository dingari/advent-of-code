use regex::{Regex, Match};
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
struct P {
    x: i32,
    y: i32,
    z: i32,
}

impl std::ops::Add for P {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn parse_input(input: &str) -> Vec<P> {
    let re = Regex::new(r"x=(-?\d+).*y=(-?\d+).*z=(-?\d+)").unwrap();

    input
        .trim_end_matches('\n')
        .split('\n')
        .map(|s: &str| -> P {
            let to_int = |c: Option<Match>| c.unwrap().as_str().parse::<i32>().unwrap();

            match re.captures(s) {
                Some(cap) => P {
                    x: to_int(cap.get(1)),
                    y: to_int(cap.get(2)),
                    z: to_int(cap.get(3)),
                },
                None => panic!("Failed to parse moon positions: {}", s),
            }
        })
        .collect()
}

fn calc_energy(pos: &Vec<P>, num_steps: usize) -> i32 {
    let (final_pos, final_vel) = (0..num_steps)
        .fold((pos.clone(), vec![P { x: 0, y: 0, z: 0 }; pos.len()]), |(p, v), _| {
            let i = p.iter()
                .permutations(2)
                .collect::<Vec<_>>()
                .chunks(3)
                .zip(&v)
                .map(|(p, &v)| {
                    let vel = p.iter().fold(v, |vel, m| {
                        let g = |a, b| if a == b { 0 } else if a < b { 1 } else { -1 };

                        let new_vel = P {
                            x: g(m[0].x, m[1].x),
                            y: g(m[0].y, m[1].y),
                            z: g(m[0].z, m[1].z),
                        };

                        vel + new_vel
                    });

                    (*p[0][0] + vel, vel)
                })
                .collect::<Vec<_>>();

            (
                i.iter().map(|(p, _)| *p).collect(),
                i.iter().map(|(_, v)| *v).collect()
            )
        });

    let e = |p: &P| p.x.abs() + p.y.abs() + p.z.abs();

    final_pos.iter().zip(&final_vel).map(|(p, v)| e(p) * e(v)).sum()
}

pub fn run(input_str: &str) {
    println!("\n-- Day 12 --");

    assert_eq!(179, calc_energy(&parse_input("<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>"), 10));
    assert_eq!(1940, calc_energy(&parse_input("<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>"), 100));

    println!("Part 1: {}", calc_energy(&parse_input(input_str), 1000));
}