pub fn run() {
    let boarding_passes: Vec<&str> = include_str!("input.txt").lines().collect();
    let mut max_sid = 0;

    for bp in boarding_passes {
        let (r, c) = bp.split_at(7);
        let sid = find_number_from_sequence(r, 127) * 8 + find_number_from_sequence(c, 7);

        if max_sid < sid {
            max_sid = sid;
        }
    }

    println!("Day05 - Part 1: {}", max_sid);
}

fn find_number_from_sequence(seq: &str, max: i32) -> i32 {
    let mut lower = 0;
    let mut upper = max;

    for c in seq.chars() {
        let step = (upper-lower)/2;
        match c {
            'F' | 'L' => upper = upper - (step+1),
            'B' | 'R' => lower = lower + (step+1),
            _ => panic!("Unknown sequence")
        }
    }

    assert_eq!(lower, upper);

    lower
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_row_from_sequence() {
        let num = find_number_from_sequence("BFFFBBF", 127);

        assert_eq!(70, num);
    }

    #[test]
    fn test_find_column_from_sequence() {
        let num = find_number_from_sequence("RLL", 7);

        assert_eq!(4, num);
    }

}