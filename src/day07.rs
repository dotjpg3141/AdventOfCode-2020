use std::{collections::HashMap, error::Error};

use crate::parse_lines;

pub fn run() -> Result<(), Box<dyn Error>> {
    let rules = parse_lines(7, |line| parse_rule(&line))?
        .into_iter()
        .collect::<HashMap<_, _>>();

    let result = rules
        .keys()
        .filter(|rule| contains_bag_recursive(&rules, *rule, "shiny gold"))
        .count()
        - 1;

    println!("Day 7a: {}", result);

    let result = count_bag_recursive(&rules, "shiny gold") - 1;
    println!("Day 7b: {}", result);

    Ok(())
}

fn contains_bag_recursive(
    rules: &HashMap<String, Vec<(usize, String)>>,
    current_bag: &str,
    target: &str,
) -> bool {
    if current_bag == target {
        return true;
    }

    rules[current_bag]
        .iter()
        .any(|(_, bag)| contains_bag_recursive(rules, bag, target))
}

fn count_bag_recursive(rules: &HashMap<String, Vec<(usize, String)>>, current_bag: &str) -> usize {
    rules[current_bag]
        .iter()
        .map(|(count, bag)| count * count_bag_recursive(rules, bag))
        .sum::<usize>()
        + 1
}

fn parse_rule(s: &str) -> (String, Vec<(usize, String)>) {
    let mut split = s.split(" contain ");
    let bag = parse_bag(split.next().expect("bag"));
    let children = split
        .next()
        .expect("children")
        .split(", ")
        .map(parse_child)
        .filter(|(count, _)| *count != 0)
        .collect::<Vec<_>>();
    debug_assert!(split.next().is_none());
    return (bag, children);

    fn parse_bag(mut s: &str) -> String {
        loop {
            let old = s;
            s = s.trim();
            s = s.trim_end_matches(".");
            s = s.trim_end_matches("bags");
            s = s.trim_end_matches("bag");
            if old == s {
                break;
            }
        }

        s.to_string()
    }

    fn parse_child(s: &str) -> (usize, String) {
        let s = s.trim();
        let idx = s.match_indices(" ").next().expect("space").0;
        let (count, bag) = s.split_at(idx);
        (parse_count(count), parse_bag(bag))
    }

    fn parse_count(s: &str) -> usize {
        match s.trim() {
            "no" => 0,
            s => s.parse::<usize>().expect("count"),
        }
    }
}
