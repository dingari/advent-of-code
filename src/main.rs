use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn read_file_to_string(file_path: &Path) -> String {
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();

    let _len = file.read_to_string(&mut contents).unwrap();

    contents
}

fn parse_lines<T, F>(input: &str, parser: F) -> Vec<T> where F: Fn(&str) -> T {
    input.split("\n").map(|s| parser(s)).collect()
}

fn main() {
    let root = env::current_dir().unwrap().join("src");
    println!("Current dir: {:?}", root.to_str());

    day1::run(&read_file_to_string(root.join("day1").join("input.txt").into_boxed_path().as_ref()));
    day2::run(&read_file_to_string(root.join("day2").join("input.txt").into_boxed_path().as_ref()));
    day3::run(&read_file_to_string(root.join("day3").join("input.txt").into_boxed_path().as_ref()));
    day4::run(&read_file_to_string(root.join("day4").join("input.txt").into_boxed_path().as_ref()));
    day5::run(&read_file_to_string(root.join("day5").join("input.txt").into_boxed_path().as_ref()));
    day6::run(&read_file_to_string(root.join("day6").join("input.txt").into_boxed_path().as_ref()));
}
