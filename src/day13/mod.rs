use std::collections::HashMap;

use super::intcode::*;
use super::parse_intcode_program;
use itertools::Itertools;

type P = (i64, i64);

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile { Empty, Wall, Block, Paddle, Ball }

fn make_tile(t: i64) -> Tile {
    match t {
        0 => Tile::Empty,
        1 => Tile::Wall,
        2 => Tile::Block,
        3 => Tile::Paddle,
        4 => Tile::Ball,
        i => panic!("Unkown tile type: {}", i),
    }
}

fn count_blocks(input: &Program) -> usize {
    let mut computer = Intcode::new(&input, None);

    computer
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.collect::<Vec<_>>())
        .filter(|v| make_tile(v[2]) == Tile::Block)
        .count()
}

pub fn run(input_str: &str) {
    let input = parse_intcode_program(input_str);

    println!("Part 1: {}", count_blocks(&input));

    let input = input.iter().take(2)
}