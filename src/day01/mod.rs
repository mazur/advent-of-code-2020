use crate::util;

pub fn run() {
    let input = read_input_data();

    let part1 = find_2020_2sum(&input);
    let part2 = find_2020_3sum(&input);

    println!("Day01 - Part 1: {}", util::multiply(&part1));
    println!("Day01 - Part 2: {}", util::multiply(&part2));
}

fn find_2020_2sum(input: &Vec<i32>) -> [i32; 2] {
    for first in input.iter() {
        for second in input.iter() {
            if first + second == 2020 {
                return [*first, *second];
            }
        }
    }
    panic!("No numbers equal the year of the palgue!")
}

fn find_2020_3sum(input: &Vec<i32>) -> [i32; 3] {
    for first in input.iter() {
        for second in input.iter() {
            for third in input.iter() {
                if first + second + third == 2020 {
                    return [*first, *second, *third];
                }
            }
        }
    }
    panic!("No numbers equal the year of the palgue!")
}

fn read_input_data() -> Vec<i32> {
    include_str!("input.txt")
        .lines()
        .map(|i| i.parse().expect("Could not parse value"))
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_2020_2sum() {
        let input = vec![1721, 979, 366, 299, 675, 1456];

        let res = find_2020_2sum(&input);

        assert_eq!(res[0] + res[1], 2020);
    }
}