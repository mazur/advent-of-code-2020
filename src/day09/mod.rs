pub fn run() {
    let input: Vec<u64> = include_str!("input.txt")
                                            .lines()
                                            .map(|l| l.parse().expect("Not a number."))
                                            .collect();

    let invalid = get_first_invalid(input.clone(), 25);
    let mut sum_set = find_continous_set(input.clone(), invalid);

    sum_set.sort();

    let result = sum_set[0] + sum_set[sum_set.len()-1];

    println!("Day09 - Part 1: {}", invalid);
    println!("Day09 - Part 2: {}", result);
}

fn find_continous_set(input: Vec<u64>, sum: u64) -> Vec<u64> {
    let n = input.len();
    
    for i in 0..n {
        let mut current = input[i];

        for j in (i+1)..n {
            if current == sum {
                return input[i..j].to_vec();
            }

            if current > sum {
                break;
            }

            current += input[j];
        }
    }

    panic!("Did not find set");
}

fn get_first_invalid(input: Vec<u64>, step: usize) -> u64 {
    let mut i = step;

    while is_valid_number(input[(i-step)..i].to_vec(), input[i]) {
        i += 1;
    }

    input[i]
}

fn is_valid_number(mut prev: Vec<u64>, num: u64) -> bool{
    prev.sort();
    let mut l = 0;
    let mut r = prev.len()-1;

    while l != r {    
        let sum = prev[l] + prev[r];
        
        if sum == num {
            return true;
        }
        else if sum > num {
            r -= 1;
        }
        else {
            l += 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_number() {
        let valid = is_valid_number(vec![35, 20, 15, 25, 47], 40);

        assert_eq!(true, valid);
    }

    #[test]
    fn test_is_not_valid_number() {
        let valid = is_valid_number(vec![95, 102, 117, 150, 182], 127);

        assert_eq!(false, valid);
    }

    #[test]
    fn test_find_first_invalid() {
        let invalid = get_first_invalid(vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576], 5);

        assert_eq!(127, invalid);
    }

    #[test]
    fn test_find_continous_set() {
        let set = find_continous_set(vec![35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576], 127);

        assert_eq!(15, set[0]);
        assert_eq!(40, set[set.len()-1]);
    }  
}