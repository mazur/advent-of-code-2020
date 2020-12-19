#[macro_use] extern crate lazy_static;
extern crate regex;

use std::env;

mod util;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day15;
mod day16;
mod day17;
mod day19;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: i32 = args[1].parse()
                            .expect("No day specified in command line.");

    match day {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        4 => day04::run(),
        5 => day05::run(),
        6 => day06::run(),
        7 => day07::run(),
        8 => day08::run(),
        9 => day09::run(),
        10 => day10::run(),
        11 => day11::run(),
        12 => day12::run(),
        15 => day15::run(),
        16 => day16::run(),
        17 => day17::run(),
        19 => day19::run(),
        _ => panic!()
    }
}
