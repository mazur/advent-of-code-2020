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

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: i32 = args[1].parse()
                            .expect("blah blah");

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
        _ => panic!()
    }
}
