use itertools::Itertools;

pub fn run(input_str: &String) {
    println!("\n-- Day 4 --");

    let v = input_str.trim_end_matches('\n').split('-').collect::<Vec<&str>>();
    let (begin, end) = (v[0].parse::<usize>().unwrap(), v[1].parse::<usize>().unwrap());

    //==============================================================================================
    // Part 1
    let is_valid_password = |pw: &str| -> bool {
        let it = pw[0..pw.len() - 1].chars()
            .zip(pw[1..pw.len()].chars());

        let has_duplicate = it.clone().any(|(x, y)| x == y);
        let is_ascending = it.clone().all(|(x, y)| x <= y);

        pw.len() == 6 && has_duplicate && is_ascending
    };

    assert_eq!(is_valid_password("111111"), true);
    assert_eq!(is_valid_password("223450"), false);
    assert_eq!(is_valid_password("123789"), false);

    let num_valid_passwords = (begin..=end)
        .fold(0_usize, |acc, x| {
            if is_valid_password(&x.to_string()) { acc + 1 } else { acc }
        });

    println!("Part 1: {}", num_valid_passwords);

    //==============================================================================================
    // Part 2
    let is_valid_password_2 = |pw: &str| -> bool {
        let groups = pw.chars()
            .group_by(|&x| x)
            .into_iter()
            .map(|(_, r)| r.collect())
            .collect::<Vec<String>>();

        let has_strict_duplicate = groups.iter().any(|s| s.len() == 2);

        is_valid_password(&pw) && has_strict_duplicate
    };

    assert_eq!(is_valid_password_2("112233"), true);
    assert_eq!(is_valid_password_2("123444"), false);
    assert_eq!(is_valid_password_2("111122"), true);

    let num_valid_passwords_2 = (begin..=end)
        .fold(0_usize, |acc, x| {
            if is_valid_password_2(&x.to_string()) { acc + 1 } else { acc }
        });

    println!("Part 2: {}", num_valid_passwords_2);
}

