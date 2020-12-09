use std::{collections::VecDeque, error::Error};

use crate::try_parse_lines;

pub fn run() -> Result<(), Box<dyn Error>> {
    let numbers = try_parse_lines(9, |line| line.parse::<u64>())?;

    let invalid_number = find_invalid_number(&numbers);
    println!("Day 9a: {}", invalid_number);

    let sums = numbers.iter().copied().scan(0, |a, b| {
        *a += b;
        Some(*a)
    });
    let sums = Some(0).into_iter().chain(sums).collect::<Vec<_>>();

    let mut found = None;
    'outer: for i in 0..sums.len() - 1 {
        for j in i + 2..sums.len() {
            let delta = sums[j] - sums[i];
            if delta > invalid_number {
                break;
            }
            if delta == invalid_number {
                found = Some((i, j));
                break 'outer;
            }
        }
    }

    let (i, j) = found.expect("valid puzzle input");
    let range = numbers[i..j].iter().copied();
    let min = range.clone().min().unwrap();
    let max = range.max().unwrap();
    let result = min + max;
    println!("Day 9b: {}", result);

    Ok(())
}

fn find_invalid_number(input: &[u64]) -> u64 {
    let mut preamble = input.iter().copied().take(25).collect::<VecDeque<_>>();
    for num in input.iter().copied().skip(preamble.len()) {
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
            return num;
        }

        preamble.pop_front();
        preamble.push_back(num);
    }

    unreachable!("invalid puzzle input")
}
