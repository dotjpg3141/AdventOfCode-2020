use std::{collections::HashMap, error::Error};

pub fn run() -> Result<(), Box<dyn Error>> {
    let instructions = std::fs::read_to_string("./input/input14.txt")?
        .lines()
        .map(parse_instruction)
        .collect::<Vec<_>>();

    let mut mask = Mask::default();
    let mut memory = HashMap::new();

    for insn in instructions.iter().cloned() {
        match insn {
            Instruction::SetMask(new_mask) => {
                mask = new_mask;
            }
            Instruction::Write(idx, value) => {
                memory.insert(idx, mask.apply(value));
            }
        }
    }

    let result = memory.values().cloned().sum::<u64>();
    println!("Day 14a: {}", result);

    Ok(())
}

#[derive(Debug, Clone, Default)]
struct Mask {
    zero: u64,
    one: u64,
}

impl Mask {
    fn apply(&self, mut value: u64) -> u64 {
        value &= self.zero;
        value |= self.one;
        value
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    SetMask(Mask),
    Write(usize, u64),
}

fn parse_instruction(s: &str) -> Instruction {
    if s.starts_with("mask") {
        let s = s.trim_start_matches("mask = ");
        Instruction::SetMask(parse_mask(s))
    } else {
        let mut s = s.trim_start_matches("mem[").split("] = ");
        let idx = s.next().expect("index").parse().expect("index number");
        let value = s.next().expect("value").parse().expect("value number");
        Instruction::Write(idx, value)
    }
}

fn parse_mask(s: &str) -> Mask {
    let zero = s
        .chars()
        .map(|c| if c == '0' { '0' } else { '1' })
        .collect::<String>();

    let one = s
        .chars()
        .map(|c| if c == '1' { '1' } else { '0' })
        .collect::<String>();

    Mask {
        zero: u64::from_str_radix(&zero, 2).unwrap(),
        one: u64::from_str_radix(&one, 2).unwrap(),
    }
}
