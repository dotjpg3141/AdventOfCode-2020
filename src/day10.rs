use std::error::Error;

use crate::try_parse_lines;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut adapters = try_parse_lines(10, |line| line.parse::<usize>())?;

    adapters.sort();

    let mut current_jolts = 0;
    let mut delta_count = [0; 4];

    for adapter in adapters.iter().copied() {
        let delta = adapter - current_jolts;
        if delta < delta_count.len() {
            delta_count[delta] += 1;
            current_jolts = adapter;
        } else {
            unreachable!("cannot connect all adapters");
        }
    }

    delta_count[3] += 1;
    let result = delta_count[1] * delta_count[3];
    println!("Day 10a: {}", result);

    adapters.insert(0, 0);
    adapters.push(current_jolts + 3);

    let mut solutions = vec![0u64; adapters.len()];
    for (idx, adapter) in adapters.iter().copied().enumerate().rev() {
        let candidates = adapters[idx + 1..]
            .iter()
            .take_while(|next_adapter| (*next_adapter - adapter) <= 3)
            .count();

        let solution = match candidates {
            0 => 1,
            1 => solutions[idx + 1],
            2 => solutions[idx + 1] + solutions[idx + 2],
            3 => solutions[idx + 1] + solutions[idx + 2] + solutions[idx + 3],
            _ => unreachable!(),
        };

        solutions[idx] = solution;
    }

    let solution = solutions[0];
    println!("Day 10b: {}", solution);

    Ok(())
}
