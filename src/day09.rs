use std::{collections::VecDeque, error::Error};

use crate::try_parse_lines;

pub fn run() -> Result<(), Box<dyn Error>> {
    let numbers = try_parse_lines(9, |line| line.parse::<u64>())?;

    let mut preamble = numbers.iter().copied().take(25).collect::<VecDeque<_>>();
    for num in numbers.iter().copied().skip(preamble.len()) {
        let mut is_valid = false;
        'validation: for (idx_a, a) in preamble.iter().copied().enumerate() {
            for b in preamble.iter().skip(idx_a + 1).copied() {
                if a + b == num {
                    is_valid = true;
                    break 'validation;
                }
            }
        }

        if !is_valid {
            println!("Day 9a: {}", num);
            break;
        }

        preamble.pop_front();
        preamble.push_back(num);
    }

    Ok(())
}
