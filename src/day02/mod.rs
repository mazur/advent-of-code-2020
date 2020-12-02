pub fn run() {
    let input: Vec<&str> = include_str!("input.txt")
                                .lines()
                                .collect();

    let mut valid_count: i32  = 0;

    for entry in input.iter() {
        let rules_password: Vec<&str> = entry.split(|c| c == '-' || c == ' ').collect();
        
        let key = rules_password[2].chars().next().unwrap();
        let min = rules_password[0].parse().expect("Could not parse min.");
        let max = rules_password[1].parse().expect("Could not parse max.");
        let password = rules_password[3];


        if is_valid_password(key, min, max, password) {
            valid_count += 1;
        }
    }

    println!("{}", valid_count);
}

fn is_valid_password(key: char, min: usize, max: usize, password: &str) -> bool{
    let count = password.matches(key).count();

    count >= min && count <= max
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