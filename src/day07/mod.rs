use std::collections::HashMap;
use itertools::Itertools;

pub fn run() {
    println!("Day07");
}

struct BagSystem<'a> {
    bags: HashMap<&'a str, Vec<&'a str>>
}

impl<'a> BagSystem<'a> {
    fn new() -> Self {
        Self {  
            bags: HashMap::new()
        }
    }

    fn count_contains(&self, key: &str) -> usize {
        match self.bags.get(key) {
            Some(b) => {
                b.iter().fold(b.len(), | count, bag | count + BagSystem::count_contains(self, bag))
            },
            None => 0
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

        if rules != "no other bags." {
            let contained_bags: Vec<&str> = rules.split(", ").map(|c| { 
                let end = c.find(" bag").unwrap();
                &c[2..end]
            }).collect();
            
            self.add_bag(bag, contained_bags);
        }
    }

    fn add_bag(&mut self, bag: &'a str, contained_in: Vec<&'a str>) {
        for con_bag in contained_in {
            let contains = self.bags
                                .entry(con_bag)
                                .or_insert(Vec::new());
            
            contains.push(bag);
        }
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
        let bags = vec!["shiny magenta", "wavy teal"];


        system.add_bag("light red", bags);

        assert_eq!(2, system.bags.len());
    }

    #[test]
    fn test_add_empty_bag_rule_to_system() {
        let mut system = BagSystem::new();

        system.add_bag_rule("striped tomato bags contain no other bags.");

        assert_eq!(0, system.bags.len());
    }

    #[test]
    fn test_add_two_bag_rule_to_system() {
        let mut system = BagSystem::new();

        system.add_bag_rule("dark orange bags contain 3 bright white bags, 4 muted yellow bags.");

        assert_eq!(2, system.bags.len());
    }

    #[test]
    fn test_build_rule_to_system_from_input() {
        let system = BagSystem::build_from_rules(TEST_INPUT);

        assert_eq!(7, system.bags.len());
    }

    #[test]
    fn test_count_bag_contains() {
        let system = BagSystem::build_from_rules(TEST_INPUT);

        let res = system.count_contains("shiny gold");

        assert_eq!(4, res);
    }
}