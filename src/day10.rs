use std::{collections::HashSet, error::Error};

use crate::try_parse_lines;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut adapters = try_parse_lines(10, |line| line.parse::<usize>())?;

    debug_assert_eq!(
        adapters.len(),
        adapters.iter().copied().collect::<HashSet<_>>().len()
    );

    adapters.sort();

    let mut current_jolts = 0;
    let mut delta_count = [0; 4];

    for adapter in adapters {
        let delta = adapter - current_jolts;
        if delta < delta_count.len() {
            delta_count[delta] += 1;
            current_jolts = adapter;
        } else {
            break;
        }
    }

    delta_count[3] += 1;
    let result = delta_count[1] * delta_count[3];
    println!("Day 10a: {}", result);

    Ok(())
}
