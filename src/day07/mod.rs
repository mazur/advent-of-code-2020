use std::collections::HashMap;
use itertools::Itertools;

pub fn run() {
    let system = BagSystem::build_from_rules(include_str!("input.txt"));
    let can_contain_shiny_gold = system.count_contains("shiny gold");
    let shiny_gold_requires_contain = system.count_required_bags("shiny gold");

    println!("Day07 - Part 1: {}", can_contain_shiny_gold);
    println!("Day07 - Part 2: {}", shiny_gold_requires_contain);
}

struct BagSystem<'a> {
    contains: HashMap<&'a str, Vec<(&'a str, u32)>>,
    contained_in: HashMap<&'a str, Vec<&'a str>>
}

impl<'a> BagSystem<'a> {
    fn new() -> Self {
        Self {  
            contains: HashMap::new(),
            contained_in: HashMap::new()
        }
    }

    fn count_required_bags(&self, key: &str) -> u32 {
        match self.contains.get(key) {
            Some (b) => {
                b.iter().fold(0, |sum, bag| sum + bag.1 + (bag.1* self.count_required_bags(bag.0)))
            },
            None => 0
        }
    }

    fn count_contains(&self, key: &str) -> usize {
        let mut all = self.get_all_contains(key);
        all.sort();
        all.dedup();
        all.len()
    }

    fn get_all_contains(&self, key: &str) -> Vec<&str> {
        match self.contained_in.get(key) {
            Some(b) => {
                let mut res = b.to_vec();
                for contained in b {
                    res.append(&mut BagSystem::get_all_contains(self, contained));
                }
                res
            },
            None => Vec::new()
        }
    }

    fn build_from_rules(input: &'a str) -> Self {
        let mut sys = Self::new();

        for line in input.lines() {
            sys.add_bag_rule(line);
        }

        sys
    }

    fn add_bag_rule(&mut self, rule_str: &'a str) {
        let (bag, rules) = rule_str.split(" bags contain ").collect_tuple().unwrap();

        let contained_bags: Vec<(&str, u32)> = if rules != "no other bags." {
                                                    rules.split(", ").map(|c| { 
                                                        let end = c.find(" bag").unwrap();
                                                        let n = c[0..1].parse().expect("Not a number");
                                                        (&c[2..end], n)
                                                    }).collect()
                                                }
                                                else {
                                                    Vec::new()
                                                };

        self.add_bag(bag, contained_bags);
    }

    fn add_bag(&mut self, bag: &'a str, contained: Vec<(&'a str, u32)>) {
        for con_bag in &contained {
            let contained_in = self.contained_in
                                .entry(con_bag.0)
                                .or_insert(Vec::new());
            
            contained_in.push(bag);
        }

        self.contains.insert(bag, contained);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn test_add_bag_to_system() {
        let mut system = BagSystem::new();
        let bags = vec![("shiny magenta",1), ("wavy teal",2)];


        system.add_bag("light red", bags);

        assert_eq!(2, system.contained_in.len());
    }

    #[test]
    fn test_add_empty_bag_rule_to_system() {
        let mut system = BagSystem::new();

        system.add_bag_rule("striped tomato bags contain no other bags.");

        assert_eq!(0, system.contained_in.len());
    }

    #[test]
    fn test_add_two_bag_rule_to_system() {
        let mut system = BagSystem::new();

        system.add_bag_rule("dark orange bags contain 3 bright white bags, 4 muted yellow bags.");

        assert_eq!(2, system.contained_in.len());
    }

    #[test]
    fn test_build_rule_to_system_from_input() {
        let system = BagSystem::build_from_rules(TEST_INPUT);

        assert_eq!(7, system.contained_in.len());
    }

    #[test]
    fn test_get_all_bag_contains() {
        let system = BagSystem::build_from_rules(TEST_INPUT);

        let res = system.get_all_contains("shiny gold");

        assert_eq!(6, res.len());
    }

    #[test]
    fn test_count_contains() {
        let system = BagSystem::build_from_rules(TEST_INPUT);

        let res = system.count_contains("shiny gold");

        assert_eq!(4, res);
    }

    #[test]
    fn test_count_required_bags() {
        let system = BagSystem::build_from_rules(TEST_INPUT);

        let res = system.count_required_bags("shiny gold");

        assert_eq!(32, res);
    }
}