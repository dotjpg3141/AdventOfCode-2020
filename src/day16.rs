use std::{
    collections::{HashMap, HashSet},
    error::Error,
    ops::RangeInclusive,
};

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./input/input16.txt")?;
    let groups = input.split("\n\n").collect::<Vec<_>>();

    let rules = groups[0].lines().map(parse_rule).collect::<Vec<_>>();
    let my_ticket = groups[1]
        .lines()
        .nth(1)
        .map(parse_ticket)
        .expect("my ticket");
    let nearby_tickets = groups[2]
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect::<Vec<_>>();

    let result = solve1(&rules, &nearby_tickets);
    println!("Day 16a: {}", result);

    let result = solve2(&rules, nearby_tickets, my_ticket);
    println!("Day 16b: {}", result);

    Ok(())
}

fn solve1(rules: &[Rule], nearby: &[Ticket]) -> u64 {
    nearby
        .iter()
        .flat_map(|ticket| &ticket.numbers)
        .copied()
        .filter(|number| rules.iter().all(|rule| !rule.contains(*number)))
        .sum()
}

fn solve2(rules: &[Rule], nearby: Vec<Ticket>, my_ticket: Ticket) -> u64 {
    let valid_tickets = nearby.iter().filter(|ticket| ticket.is_valid(rules));

    let mut votes = vec![HashMap::<_, usize>::new(); rules.len()];
    for ticket in valid_tickets.clone() {
        for (idx, number) in ticket.numbers.iter().copied().enumerate() {
            for rule in rules.iter().filter(|rule| rule.contains(number)) {
                *votes[idx].entry(rule.name.as_str()).or_default() += 1;
            }
        }
    }

    let mut result = 1;
    let mut visited_rules = HashSet::new();
    let valid_ticket_count = valid_tickets.count();

    while let Some((idx, rule_name)) = find_unique_vote(&votes, valid_ticket_count, &visited_rules)
    {
        if rule_name.starts_with("departure") {
            result *= my_ticket.numbers[idx];
        }

        visited_rules.insert(rule_name);
    }

    debug_assert_eq!(visited_rules.len(), rules.len());

    result
}

fn find_unique_vote<'a>(
    votes: &[HashMap<&'a str, usize>],
    ticket_count: usize,
    visited_rules: &HashSet<&str>,
) -> Option<(usize, &'a str)> {
    votes.iter().enumerate().find_map(|(idx, votes)| {
        get_unique(votes.iter().filter_map(|(rule_name, count)| {
            if *count == ticket_count && !visited_rules.contains(rule_name) {
                Some((idx, *rule_name))
            } else {
                None
            }
        }))
    })
}

fn get_unique<T>(mut iter: impl Iterator<Item = T>) -> Option<T> {
    let maybe_unique = iter.next()?;
    if iter.next().is_none() {
        Some(maybe_unique)
    } else {
        None
    }
}

struct Rule {
    name: String,
    range1: RangeInclusive<u64>,
    range2: RangeInclusive<u64>,
}

impl Rule {
    fn contains(&self, value: u64) -> bool {
        self.range1.contains(&value) || self.range2.contains(&value)
    }
}

struct Ticket {
    numbers: Vec<u64>,
}

impl Ticket {
    fn is_valid(&self, rules: &[Rule]) -> bool {
        self.numbers
            .iter()
            .all(|number| rules.iter().any(|rule| rule.contains(*number)))
    }
}

fn parse_rule(s: &str) -> Rule {
    let mut parts = s.split(": ").flat_map(|s| s.split(" or "));
    let name = parts.next().expect("name").to_string();
    let range1 = parse_range(parts.next().expect("range 1"));
    let range2 = parse_range(parts.next().expect("range 2"));
    debug_assert!(parts.next().is_none());
    Rule {
        name,
        range1,
        range2,
    }
}

fn parse_range(s: &str) -> RangeInclusive<u64> {
    let mut parts = s.split('-');
    let start = parts.next().expect("start").parse().expect("start number");
    let end = parts.next().expect("end").parse().expect("end number");
    debug_assert!(parts.next().is_none());
    start..=end
}

fn parse_ticket(s: &str) -> Ticket {
    let numbers = s
        .split(',')
        .map(|s| s.parse().expect("ticket number"))
        .collect();
    Ticket { numbers }
}
