use std::collections::HashMap;

pub fn run() {
    let res_p1 = find_number_in_recitation(vec![2,0,1,9,5,19], 2020);

    println!("Day16 - Part 1: {}", res_p1);

    let res_p2 = find_number_in_recitation(vec![2,0,1,9,5,19], 30000000);

    println!("Day16 - Part 2: {}", res_p2);
}

struct MemoryGame {
    numbers: HashMap<usize, Vec<usize>>,
    round: usize
}

impl MemoryGame {
    fn new () -> Self {
        MemoryGame {
            numbers: HashMap::new(),
            round: 0
        }
    }

    fn add_next(&mut self, num: usize) -> usize {
        self.round += 1;
        let nums = self.numbers.entry(num).or_insert(Vec::new());

        nums.push(self.round);

        let i = nums.len()-1;

        if i == 0 {
            0
        }
        else {
            nums[i] - nums[i-1]
        }
    }
}

fn find_number_in_recitation(init: Vec<usize>, find: usize) -> usize {
    let mut game = MemoryGame::new();
    let mut last = 0;

    for n in init {
        last = game.add_next(n);
    }

    while game.round < find-1 {
        last = game.add_next(last);
    }

    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_number_in_recitation_1() {
        let res = find_number_in_recitation(vec![0,3 ,6], 2020);

        assert_eq!(436, res);
    }

    #[test]
    fn test_find_number_in_recitation_2() {
        let res = find_number_in_recitation(vec![1,3 ,2], 2020);

        assert_eq!(1, res);
    }

    #[test]
    fn test_find_number_in_recitation_3() {
        let res = find_number_in_recitation(vec![2,1 ,3], 2020);

        assert_eq!(10, res);
    }
}