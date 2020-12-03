use std::env;

mod day01;
mod day02;
mod day03;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: i32 = args[1].parse()
                            .expect("blah blah");

    match day {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        _ => panic!()
    }
}
