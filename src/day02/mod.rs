pub fn run() {
    let input = parse_input_data(include_str!("input.txt"));

    let mut valid_count: i32  = 0;

    for entry in input.iter() {
        if is_valid_password(entry.0, entry.1, entry.2, entry.3) {
            valid_count += 1;
        }
    }

    println!("{}", valid_count);
}

fn is_valid_password(min: usize, max: usize, key: char, password: &str) -> bool{
    let count = password.matches(key).count();

    count >= min && count <= max
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
        rules_password.next().expect("Wrong number of arguments on line.")
    )
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_be_valid_password_1() {
        let is_valid = is_valid_password('a', 1, 3, "abcde");

        assert!(is_valid);
    }

    #[test]
    fn test_should_be_valid_password_2() {
        let is_valid = is_valid_password('c', 2, 9, "ccccccccc");

        assert!(is_valid);
    }

    #[test]
    fn test_should_not_be_valid_password() {
        let is_not_valid = !is_valid_password('b', 1, 3, "cdefg");

        assert!(is_not_valid);
    }
}