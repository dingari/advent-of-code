pub fn run(input_str: &str) {
    println!("\n-- Day 8 --");

    let input: Vec<char> = input_str
        .trim_end_matches('\n')
        .chars()
        .collect();

    //==============================================================================================
    // Part 1
    let counts = input
        .chunks(25 * 6)
        .map(|s| (0..=9)
            .map(|d|
                s.iter()
                    .filter(|&&c| c == std::char::from_digit(d, 10).unwrap())
                    .count()
            )
            .collect::<Vec<usize>>()
        )
        .min_by(|v1, v2| v1[0].cmp(&v2[0]))
        .unwrap();

    println!("Part 1: {}", counts[1] * counts[2]);

    //==============================================================================================
    // Part 2
    let lines = input
        .chunks(25 * 6)
        .fold(vec!['2'; 25 * 6], |acc, x| x.iter()
            .zip(&acc)
            .map(|(&c1, &c2)| if c2 == '2' { c1 } else { c2 })
            .collect(),
        )
        .chunks(25)
        .map(|l| l
            .iter()
            .map(|&c| if c == '1' { 'x' } else { ' ' })
            .collect::<String>()
        )
        .collect::<Vec<String>>();

    println!("Part 2:");

    for line in lines {
        println!("{}", line);
    }
}