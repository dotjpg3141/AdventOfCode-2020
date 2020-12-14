use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    v1::run()?;
    v2::run()?;

    Ok(())
}

mod v1 {
    use std::{collections::HashMap, error::Error};

    pub(super) fn run() -> Result<(), Box<dyn Error>> {
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
        Write(u64, u64),
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
}

mod v2 {
    use std::error::Error;

    pub(super) fn run() -> Result<(), Box<dyn Error>> {
        let mut mask = Mask::default();
        let instructions = std::fs::read_to_string("./input/input14.txt")?
            .lines()
            .map(parse_instruction)
            .flat_map(|insn| match insn {
                Instruction::SetMask(new_mask) => {
                    mask = new_mask;
                    None
                }
                Instruction::Write(idx, value) => {
                    let address = mask.apply(idx);
                    Some((address, value))
                }
            })
            .collect::<Vec<_>>();

        let range = AddressRange {
            idx: 0,
            floating: u64::MAX,
        };

        // NOTE(jpg): solution is wrong. the example works
        let sum = sum_bits(&instructions, range, 0);
        println!("Day 14b: {} (wrong)", sum);

        Ok(())
    }

    fn sum_bits(instructions: &[(AddressRange, u64)], old_range: AddressRange, depth: u64) -> i64 {
        let mut sum = 0;

        for (idx, (new_range, new_value)) in instructions.iter().cloned().enumerate() {
            let known_bits_mask = !(old_range.floating | new_range.floating);
            let known_bits_old = old_range.idx & known_bits_mask;
            let known_bits_new = new_range.idx & known_bits_mask;

            if known_bits_old != known_bits_new {
                continue;
            }

            let new_mask = old_range.floating & new_range.floating;
            let bit_count = new_mask.count_ones();

            sum += new_value as i64 * 2i64.pow(bit_count);

            let remove_range = AddressRange {
                idx: new_range.idx,
                floating: new_mask,
            };
            sum -= sum_bits(&instructions[0..idx], remove_range, depth + 1);
        }

        sum
    }

    #[derive(Copy, Clone)]
    struct AddressRange {
        idx: u64,
        floating: u64,
    }

    #[derive(Debug, Clone, Default)]
    struct Mask {
        one: u64,
        floating: u64,
    }

    impl Mask {
        fn apply(&self, idx: u64) -> AddressRange {
            AddressRange {
                idx: idx | self.one,
                floating: self.floating,
            }
        }
    }

    #[derive(Debug, Clone)]
    enum Instruction {
        SetMask(Mask),
        Write(u64, u64),
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
        let one = s
            .chars()
            .map(|c| if c == '1' { '1' } else { '0' })
            .collect::<String>();

        let floating = s
            .chars()
            .map(|c| if c == 'X' { '1' } else { '0' })
            .collect::<String>();

        Mask {
            one: u64::from_str_radix(&one, 2).unwrap(),
            floating: u64::from_str_radix(&floating, 2).unwrap(),
        }
    }
}
