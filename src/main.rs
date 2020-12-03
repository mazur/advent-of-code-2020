use std::env;

mod day01;
mod day02;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: i32 = args[1].parse()
                            .expect("blah blah");

    match day {
        1 => day01::run(),
        2 => day02::run(),
        _ => panic!()
    }
}
