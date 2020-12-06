pub fn run() {
    let groups: Vec<&str> = include_str!("input.txt").split("\n\n").collect();
    let mut some = 0;
    let mut all = 0;

    for g in groups.iter() {
        let a = count_answers_in_group(g);
        some += a.0;
        all += a.1;
    }

    println!("Day06 - Part 1: {}", some);
    println!("Day06 - Part 2: {}", all);
}

fn count_answers_in_group(forms: &str) -> (i32, i32) {
    let chars: Vec<char> = forms.chars().collect();
    let mut count: [i32; 26] = [0; 26];
    let mut num_of_groups = 1;

    for c in chars.iter() {
        if c.is_whitespace() {
            num_of_groups += 1;
        }
        else {
            count[(*c as usize)-97] += 1;
        }
    }

    let mut some = 0;
    let mut all = 0;

    for c in count.iter() {
        if *c > 0 {
            some += 1;
        }
        if *c == num_of_groups {
            all += 1;
        }
    }

    (some, all)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_answers_in_group() {
        let count = count_answers_in_group("ab
ac");
        assert_eq!(3, count.0);
        assert_eq!(1, count.1);
    }
}