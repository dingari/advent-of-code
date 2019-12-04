use std::collections::HashSet;

use regex::Regex;

type Path = HashSet<(i32, i32)>;
type Point = (i32, i32);

fn parse_input(input: &String) -> Vec<Vec<(i32, i32)>> {
    let re = Regex::new(r"(\w)(\d+)").unwrap();

    let parse_fn = |s: &str| -> (i32, i32) {
        let cap = re.captures(s).unwrap();
        let dir = cap.get(1).unwrap().as_str();
        let len = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();

        match dir {
            "L" => (-len, 0),
            "R" => (len, 0),
            "U" => (0, len),
            "D" => (0, -len),
            _ => panic!("Unkown direction: {}", dir),
        }
    };

    input
        .trim_end_matches('\n')
        .split('\n')
        .map(|s| s.split(',').map(parse_fn).collect::<Vec<(i32, i32)>>())
        .collect()
}


fn interpolate(p0: Point, dx: i32, dy: i32) -> Vec<Point> {
    let sign = |x: i32| -> i32 { x / x.abs() };

    let mut vec: Vec<Point> = vec![];

    let (x, y) = p0;

    if dy == 0 {
        for x in num::iter::range_step(x, x + dx, sign(dx)) {
            vec.push((x, y));
        }
    } else if dx == 0 {
        for y in num::iter::range_step(y, y + dy, sign(dy)) {
            vec.push((x, y));
        }
    }

    vec
}

fn construct_path(v: &Vec<(i32, i32)>) -> Path {
    let mut ret: HashSet<(i32, i32)> = HashSet::new();

    let mut prev_p = (0_i32, 0_i32);

    for (dx, dy) in v {
        ret.extend(interpolate(prev_p, *dx, *dy).iter());

        prev_p = (prev_p.0 + *dx, prev_p.1 + *dy);
    }

    ret.remove(&(0_i32, 0_i32));

    ret
}

pub fn run(input_str: &String) {
    println!("\n-- Day 3 --");

    let manhattan_distance = |p1: Point, p2: Point| -> i32 { (p1.0 - p2.0).abs() + (p1.0 - p2.1).abs() };

    //==============================================================================================
    // Part 1
    let min_manhattan_distance = |v: &Vec<Vec<(i32, i32)>>| -> Option<i32> {
        let path_a = construct_path(&v[0]);
        let path_b = construct_path(&v[1]);

        path_a
            .intersection(&path_b)
            .map(|p| manhattan_distance((0, 0), *p))
            .min()
    };

    let test_inp_1 = parse_input(&"R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83".to_string());
    let test_inp_2 = parse_input(&"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n U98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string());
    assert_eq!(min_manhattan_distance(&test_inp_1).unwrap(), 159);
    assert_eq!(min_manhattan_distance(&test_inp_2).unwrap(), 135);

    let input = parse_input(&input_str);
    println!("Part 1: {}", min_manhattan_distance(&input).unwrap());

    //==============================================================================================
    // Part 2
    let steps_to_intersection = |steps: &Vec<(i32, i32)>, intersection: Point| -> Option<usize> {
        let mut prev_p: (i32, i32) = (0, 0);

        let mut v: Vec<(i32, i32)> = vec![];

        for (dx, dy) in steps {
            v.extend(interpolate(prev_p, *dx, *dy).iter().cloned());
            prev_p = (prev_p.0 + *dx, prev_p.1 + *dy);
        }

        v.iter().position(|x| *x == intersection)
    };

    let find_min_steps = |inp: &Vec<Vec<(i32, i32)>>| -> Option<usize> {
        let pa = construct_path(&inp[0]);
        let pb = construct_path(&inp[1]);

        let intersections = pa.intersection(&pb);

        intersections
            .map(|i| steps_to_intersection(&inp[0], *i).unwrap() + steps_to_intersection(&inp[1], *i).unwrap())
            .min()
    };

    assert_eq!(find_min_steps(&test_inp_1).unwrap(), 610);
    assert_eq!(find_min_steps(&test_inp_2).unwrap(), 410);

    println!("Part 2: {}", find_min_steps(&input).unwrap());
}

