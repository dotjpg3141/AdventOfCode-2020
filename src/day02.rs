use std::{error::Error, fs::File, io::BufRead, io::BufReader, ops::RangeInclusive, str::FromStr};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input02.txt")?;
    let reader = BufReader::new(file);

    let entries = reader
        .lines()
        .map(|line| -> Result<_, Box<dyn Error>> { Ok(line?.parse::<PasswordRecord>()?) })
        .collect::<Result<Vec<_>, _>>()?;

    let valid_count = entries.iter().filter(|record| record.is_valid()).count();
    println!("Day 2a: {}", valid_count);

    Ok(())
}

struct PasswordRecord {
    range: RangeInclusive<usize>,
    c: char,
    password: String,
}

impl PasswordRecord {
    fn is_valid(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.c).count();
        self.range.contains(&count)
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
            range: start..=end,
            c,
            password,
        })
    }
}
