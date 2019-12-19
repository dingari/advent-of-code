use itertools::Itertools;
use minifb::{Scale, Window, WindowOptions};

use super::intcode::*;
use super::parse_intcode_program;

const RED: u32 = 0xff_ff_00_00;
const GREEN: u32 = 0xff_00_ff_00;
const BLUE: u32 = 0xff_00_00_ff;
const BLACK: u32 = 0xff_00_00_00;
const WHITE: u32 = 0xff_ff_ff_ff;

//const EMPTY: usize = 0;
//const WALL: usize = 1;
const BLOCK: usize = 2;
const PADDLE: usize = 3;
const BALL: usize = 4;

const COLORS: [u32; 5] = [BLACK, WHITE, RED, GREEN, BLUE];

fn count_blocks(input: &Program) -> usize {
    Intcode::new(&input, None)
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.collect::<Vec<_>>())
        .filter(|v| v[2] as usize == BLOCK)
        .count()
}

fn calc_dimensions(input: &Program) -> (usize, usize) {
    let tiles = Intcode::new(&input, None)
        .into_iter()
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.collect::<Vec<_>>())
        .map(|v| (v[0], v[1]))
        .collect::<Vec<_>>();

    (
        *tiles.iter().map(|(x, _)| x).max().unwrap() as usize + 1,
        *tiles.iter().map(|(_, y)| y).max().unwrap() as usize + 1
    )
}

fn play_game(input: &Program) -> usize {
    let (wx, wy) = calc_dimensions(input);

    let mut window = Window::new(
        "DMG-01",
        wx,
        wy,
        WindowOptions {
            borderless: false,
            title: false,
            resize: false,
            scale: Scale::X16,
        },
    ).unwrap();

    let mut framebuffer = vec![0; wx * wy];

    let game = input.iter().enumerate().map(|(i, &x)| if i == 0 { 2 } else { x }).collect();
    let mut computer = Intcode::new(&game, None);

    let mut ball: Option<i64> = None;
    let mut paddle: Option<i64> = None;
    let mut score = 0;

    while let Some(out) = computer.run_til_num_output(3) {
        let v = out.into_iter().collect::<Vec<_>>();
        let (x, y) = (v[0], v[1]);

        if x == -1 {
            score = v[2] as usize;
        } else {
            framebuffer[(y as usize) * wx + x as usize] = COLORS[v[2] as usize];

            match v[2] as usize {
                PADDLE => paddle = Some(x),
                BALL => ball = Some(x),
                _ => {}
            }
        }

        match (paddle, ball) {
            (Some(p), Some(b)) => {
                let input = if p < b { 1 } else if p > b { -1 } else { 0 };

                computer.input.clear();
                computer.input.push_back(input);
            }
            _ => {}
        }

        window.update_with_buffer(&framebuffer).unwrap();
    }

    score
}

pub fn run(input_str: &str) {
    println!("\n-- Day 13 --");

    let input = parse_intcode_program(input_str);

    println!("Part 1: {}", count_blocks(&input));
    println!("Part 2: {}", play_game(&input));
}