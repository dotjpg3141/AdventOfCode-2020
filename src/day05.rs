use std::{error::Error, fs::File, io::BufRead, io::BufReader};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input05.txt")?;
    let reader = BufReader::new(file);

    let entries = reader
        .lines()
        .map(|line| -> Result<_, Box<dyn Error>> { Ok(determine_seat_position(line?.as_str())) })
        .collect::<Result<Vec<_>, _>>()?;

    let seat_id = entries
        .iter()
        .map(|seat| seat.id())
        .max()
        .expect("input is non-empty");

    println!("Day 5a: {}", seat_id);

    Ok(())
}

fn binary_partition(mut min: u32, mut max: u32, low: char, high: char, command: &str) -> u32 {
    for c in command.chars() {
        let mid = (min + max) / 2;
        if c == low {
            max = mid;
        } else if c == high {
            min = mid;
        }
    }

    assert_eq!(min, max - 1);
    min
}

fn determine_seat_position(command: &str) -> Seat {
    let row = binary_partition(0, 128, 'F', 'B', command);
    let col = binary_partition(0, 8, 'L', 'R', command);
    Seat { row, col }
}

struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
    fn id(&self) -> u32 {
        self.row * 8 + self.col
    }
}
