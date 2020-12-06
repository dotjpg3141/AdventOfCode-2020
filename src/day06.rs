use std::{collections::HashMap, collections::HashSet, error::Error, fs::read_to_string};

pub fn run() -> Result<(), Box<dyn Error>> {
    let result = read_to_string("./input/input06.txt")?
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| *c != '\n')
                .collect::<HashSet<_>>()
                .len()
        })
        .sum::<usize>();

    println!("Day 6a: {:#?}", result);

    let result = read_to_string("./input/input06.txt")?
        .split("\n\n")
        .map(|group| {
            let persons = group
                .split('\n')
                .filter(|s| !s.is_empty())
                .collect::<HashSet<_>>();

            let mut count: HashMap<char, usize> = HashMap::new();
            for c in persons.iter().flat_map(|s| s.chars()) {
                *count.entry(c).or_default() += 1;
            }

            count
                .values()
                .filter(|count| **count == persons.len())
                .count()
        })
        .sum::<usize>();

    println!("Day 6b: {:#?}", result);

    Ok(())
}
