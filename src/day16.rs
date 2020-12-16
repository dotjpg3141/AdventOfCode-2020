use std::{error::Error, ops::RangeInclusive};

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
