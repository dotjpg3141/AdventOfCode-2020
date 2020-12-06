use std::{collections::HashSet, error::Error, fs::read_to_string};

pub fn run() -> Result<(), Box<dyn Error>> {
    let s = read_to_string("./input/input06.txt")?
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| *c != '\n')
                .collect::<HashSet<_>>()
                .len()
        })
        .sum::<usize>();

    println!("Day 6a: {:#?}", s);

    Ok(())
}
