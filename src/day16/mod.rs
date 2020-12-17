use itertools::Itertools;

pub fn run() {
    let input = include_str!("input.txt");
    let (ruleset, _, tickets) = parse_input(input);

    let errors = ruleset.find_errors(tickets)
                        .iter()
                        .fold(0, |sum, i| sum + i);

    println!("Day16 - Part 1: {}", errors);
}

type Ticket = Vec<u32>;
type Range = (u32, u32);

struct Rule {
    id: String,
    ranges: Vec<Range>
}

impl Rule {
    fn from(rule_str: &str) -> Self {
        let (id, range_str) = rule_str.split(":").collect_tuple().unwrap();
        let ranges = range_str.split("or")
                                .map(|r| r.split("-")
                                            .map(|i| i.trim().parse().expect("Should be a number."))
                                            .collect_tuple().unwrap())
                                .collect();

        Rule {
            id: id.to_string(),
            ranges: ranges
        }
    }

    fn match_rule(&self, i: u32) -> bool {
        for r in &self.ranges {
            if r.0 <= i && r.1 >= i {
                return true
            }
        }
        false
    }
}

struct RuleSet {
    rules: Vec<Rule>
}

impl RuleSet {
    fn from(rule_str: &str) -> Self {
        let rules = rule_str.lines().map(|l| Rule::from(l)).collect();

        RuleSet {
            rules: rules
        }
    }

    fn match_rule(&self, i: u32) -> bool {
        for r in &self.rules {
            if r.match_rule(i) {
                return true
            }
        }
        false
    }

    fn find_errors(&self, tickets: Vec<Ticket>) -> Vec<u32> {
        tickets.iter().flatten()
                .filter(|&i| !self.match_rule(*i))
                .map(|&i| i)
                .collect()
    }
}

fn parse_ticket(ticket_str: &str) -> Ticket {
    ticket_str.split(",").map(|i| i.parse().expect("Should be a number.")).collect()
}

fn parse_input(input: &str) -> (RuleSet, Ticket, Vec<Ticket>) {
    let (rule_str, my_ticket_str, tickets_str) = input.split("\n\n").collect_tuple().expect("Could not find three sections!");

    (RuleSet::from(rule_str), 
    parse_ticket(my_ticket_str.lines().nth(1).expect("Failed attempt to skip label")),
    tickets_str.lines().skip(1).map(|l| parse_ticket(l)).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rule_from_string() {
        let rule = Rule::from("departure location: 40-152 or 161-969");

        assert_eq!("departure location", rule.id);
        assert_eq!(2, rule.ranges.len());
        assert_eq!((40, 152), rule.ranges[0]);
        assert_eq!((161, 969), rule.ranges[1]);
    }

    #[test]
    fn test_ticket_from_string() {
        let ticket = parse_ticket("139,109,61,149,101,89,103,53,107,59,73,151,71,67,97,113,83,163,137,167");

        assert_eq!(vec![139,109,61,149,101,89,103,53,107,59,73,151,71,67,97,113,83,163,137,167], ticket);
    }

    #[test]
    fn test_ruleset_from_string() {
        let ruleset = RuleSet::from("departure location: 40-152 or 161-969\ndeparture station: 39-838 or 845-971\ndeparture platform: 39-209 or 217-970");

        assert_eq!(3, ruleset.rules.len());
    }

    #[test]
    fn test_parse_input_string() {
        let (ruleset, my_ticket, tickets) = parse_input("class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12");

        assert_eq!(3, ruleset.rules.len());
        assert_eq!(vec![7,1,14], my_ticket);
        assert_eq!(4, tickets.len());
    }

    #[test]
    fn test_find_errors() {
        let (ruleset, _, tickets) = parse_input("class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12");

        let errors = ruleset.find_errors(tickets);

        assert_eq!(71, errors.iter().fold(0, |sum, i| sum + i));
    }
}