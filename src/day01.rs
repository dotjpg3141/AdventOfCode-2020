use std::{error::Error, fs::File, io::BufRead, io::BufReader};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input01.txt")?;
    let reader = BufReader::new(file);

    let entries = reader
        .lines()
        .map(|line| -> Result<_, Box<dyn Error>> { Ok(line?.parse::<u64>()?) })
        .collect::<Result<Vec<_>, _>>()?;

    let mut result = None;
    'outer1: for (idx_a, a) in entries.iter().copied().enumerate() {
        for (idx_b, b) in entries.iter().copied().enumerate() {
            if idx_a != idx_b && a + b == 2020 {
                result = Some(a * b);
                break 'outer1;
            }
        }
    }
    println!("Day1 a: {:?}", result);

    let mut result = None;
    'outer2: for (idx_a, a) in entries.iter().copied().enumerate() {
        for (idx_b, b) in entries.iter().copied().enumerate() {
            for (idx_c, c) in entries.iter().copied().enumerate() {
                if idx_a != idx_b && idx_a != idx_c && idx_b != idx_c && a + b + c == 2020 {
                    result = Some(a * b * c);
                    break 'outer2;
                }
            }
        }
    }
    println!("Day1 b: {:?}", result);

    Ok(())
}
