use itertools::Itertools;

pub fn run(input_str: &String) {
    println!("\n-- Day 4 --");

    let v = input_str.trim_end_matches('\n').split('-').collect::<Vec<&str>>();
    let (begin, end) = (v[0].parse::<usize>().unwrap(), v[1].parse::<usize>().unwrap());

    let count_valid_passwords = |begin: usize, end: usize, f: &dyn Fn(&str) -> bool| -> usize {
        (begin..=end)
            .filter(|x| f(&x.to_string()))
            .count()
    };

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

    println!("Part 1: {}", count_valid_passwords(begin, end, &is_valid_password));

    //==============================================================================================
    // Part 2
    let is_valid_password_2 = |pw: &str| -> bool {
        let has_strict_duplicate = pw.chars()
            .group_by(|&x| x)
            .into_iter()
            .map(|(_, r)| r.collect::<String>())
            .any(|s| s.len() == 2);

        is_valid_password(&pw) && has_strict_duplicate
    };

    assert_eq!(is_valid_password_2("112233"), true);
    assert_eq!(is_valid_password_2("123444"), false);
    assert_eq!(is_valid_password_2("111122"), true);

    println!("Part 2: {}", count_valid_passwords(begin, end, &is_valid_password_2));
}

