use std::{error::Error, str::FromStr};

use crate::try_parse_lines;

pub fn run() -> Result<(), Box<dyn Error>> {
    let entries = try_parse_lines(2, |line| line.parse::<PasswordRecord>())?;

    let valid_count = entries.iter().filter(|record| record.is_valid_a()).count();
    println!("Day 2a: {}", valid_count);

    let valid_count = entries.iter().filter(|record| record.is_valid_b()).count();
    println!("Day 2b: {}", valid_count);

    Ok(())
}

struct PasswordRecord {
    start: usize,
    end: usize,
    c: char,
    password: String,
}

impl PasswordRecord {
    fn is_valid_a(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.c).count();
        (self.start..=self.end).contains(&count)
    }

    fn is_valid_b(&self) -> bool {
        let c1 = self.password.chars().nth(self.start - 1);
        let c2 = self.password.chars().nth(self.end - 1);
        (c1 == Some(self.c)) ^ (c2 == Some(self.c))
    }
}

impl FromStr for PasswordRecord {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(|c| c == '-' || c == ' ' || c == ':');
        let start = split.next().expect("start").parse::<usize>()?;
        let end = split.next().expect("end").parse::<usize>()?;
        let c = split.next().expect("c").parse::<char>()?;
        let _ = split.next().expect("_");
        let password = split.next().expect("password").parse::<String>()?;

        Ok(PasswordRecord {
            start,
            end,
            c,
            password,
        })
    }
}
