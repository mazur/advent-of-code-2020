use std::fs;

pub fn run() {
    let input = read_input_data();

    let result = find_2020(input);  
    
    let solution1 = result.0 * result.1;

    println!("Day01 - Solution 1: {}", solution1);
}

fn find_2020(input: Vec<i32>) -> (i32, i32) {
    for first in input.iter() {
        for second in input.iter() {
            if first + second == 2020 {
                return (*first, *second);
            }
        }
    }
    panic!("No numbers equal the year of the palgue!")
}

fn read_input_data() -> Vec<i32> {
    fs::read_to_string("day01/input.txt")
        .expect("Could not read input file")
        .lines()
        .map(|i| i.parse().expect("Could not parse value"))
        .collect();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_2020() {
        let input = vec![1721, 979, 366, 299, 675, 1456];

        let res = find_2020(input);

        assert_eq!(res.0 + res.1, 2020);
    }
}