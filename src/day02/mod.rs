pub fn run() {
    let input = parse_input_data(include_str!("input.txt"));

    let mut valid_count_old: i32  = 0;
    let mut valid_count_new: i32  = 0;

    for entry in input.iter() {
        if is_valid_password_old(entry.0, entry.1, entry.2, entry.3) {
            valid_count_old += 1;
        }
        if is_valid_password_new(entry.0, entry.1, entry.2, entry.3) {
            valid_count_new += 1;
        }
    }

    println!("Day02 - Part 1: {}", valid_count_old);
    println!("Day02 - Part 2: {}", valid_count_new);
}

fn is_valid_password_old(min: usize, max: usize, key: char, password: &str) -> bool{
    let count = password.matches(key).count();

    count >= min && count <= max
}

fn is_valid_password_new(min: usize, max: usize, key: char, password: &str) -> bool{
    let chars: Vec<char> = password.chars().collect();

    (chars[min-1] == key) ^ (chars[max-1] == key)
}

fn parse_input_data(input: &str) -> Vec<(usize, usize, char, &str)> {
    input.lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> (usize, usize, char, &str) {
  
    let mut rules_password = line.split(|c| c == '-' || c == ' ');
    
    (
        rules_password
            .next().expect("Wrong number of arguments on line.")
            .parse().expect("Could not parse min."),
        rules_password
            .next().expect("Wrong number of arguments on line.")
            .parse().expect("Could not parse max."),
        rules_password
            .next().expect("Wrong number of arguments on line.")
            .chars().next().unwrap(),
        rules_password.next().expect("Wrong number of arguments on line.").trim()
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_be_valid_password_old_1() {
        let is_valid = is_valid_password_old(1, 3, 'a', "abcde");

        assert!(is_valid);
    }

    #[test]
    fn test_should_be_valid_password_old_2() {
        let is_valid = is_valid_password_old(2, 9, 'c', "ccccccccc");

        assert!(is_valid);
    }

    #[test]
    fn test_should_not_be_valid_password_old() {
        let is_not_valid = !is_valid_password_old(1, 3, 'b', "cdefg");

        assert!(is_not_valid);
    }

    #[test]
    fn test_should_be_valid_password_new() {
        let is_valid = is_valid_password_new(1, 3, 'a', "abcde");

        assert!(is_valid);
    }

    #[test]
    fn test_should_not_be_valid_password_new_1() {
        let is_not_valid = !is_valid_password_new(2, 9, 'c', "ccccccccc");

        assert!(is_not_valid);
    }

    #[test]
    fn test_should_not_be_valid_password_new_2() {
        let is_not_valid = !is_valid_password_new(1, 3, 'b', "cdefg");

        assert!(is_not_valid);
    }
}