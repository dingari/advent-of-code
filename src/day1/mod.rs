use super::parse_lines;

fn calculate_fuel(x: &i32) -> i32 { x / 3 - 2 }

fn calculate_fuel_rec(x: &i32) -> i32 {
    let f = calculate_fuel(x);
    if f <= 0 { 0 } else { f + calculate_fuel_rec(&f) }
}

pub fn run(input_str: &String) {
    println!("\n-- Day 1 --");

    let input = parse_lines(
        &input_str.trim_end_matches('\n').to_string(),
        |s| s.parse::<i32>().unwrap(),
    );

    // Part 1
    assert_eq!(calculate_fuel(&12), 2);
    assert_eq!(calculate_fuel(&14), 2);
    assert_eq!(calculate_fuel(&1969), 654);
    assert_eq!(calculate_fuel(&100756), 33583);

    let sum1: i32 = input.iter().map(calculate_fuel).sum();
    println!("Part 1: {:?}", sum1);

    // Part 2
    assert_eq!(calculate_fuel_rec(&14), 2);
    assert_eq!(calculate_fuel_rec(&1969), 966);

    let sum2: i32 = input.iter().map(calculate_fuel_rec).sum();
    println!("Part 2: {:?}", sum2);
}