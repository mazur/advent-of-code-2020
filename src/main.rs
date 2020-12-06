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
        _ => panic!()
    }
}
