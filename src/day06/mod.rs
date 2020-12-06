pub fn run() {
    let groups: Vec<&str> = include_str!("input.txt").split("\n\n").collect();

    let sum_of_unique = groups.iter().fold(0, | sum, g | sum + count_chars_in_group(g));

    println!("Day06 - Part 1: {}", sum_of_unique);
}

fn count_chars_in_group(forms: &str) -> i32 {
    let mut sorted: Vec<char> = forms.chars().collect();
    sorted.sort();
    sorted.dedup();
    sorted.retain(|c| !c.is_whitespace());

    sorted.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_chars_in_string() {
        let count = count_chars("ab
ac");
        assert_eq!(3, count);
    }
}