use std::{collections::HashMap, collections::HashSet, error::Error};

use crate::parse_lines;

pub fn run() -> Result<(), Box<dyn Error>> {
    let seats = parse_lines(5, |line| determine_seat_position(&line))?;

    let seat_id = seats
        .iter()
        .map(|seat| seat.id())
        .max()
        .expect("input is non-empty");

    println!("Day 5a: {}", seat_id);

    let mut seat_neighbours: HashMap<_, u32> = HashMap::new();
    for Seat { row, col } in seats.iter().cloned() {
        let neighbours = [
            Seat { row: row - 1, col },
            Seat { row: row + 1, col },
            Seat { row, col: col - 1 },
            Seat { row, col: col + 1 },
        ];

        for n in &neighbours {
            *seat_neighbours.entry(n.clone()).or_default() += 1;
        }
    }

    let all_seats = seats.into_iter().collect::<HashSet<_>>();
    let my_seat = seat_neighbours
        .into_iter()
        .filter(|(seat, count)| *count == 4 && !all_seats.contains(seat))
        .map(|(seat, _)| seat.id())
        .collect::<Vec<_>>();

    println!("Day 5b: {:?}", my_seat);

    Ok(())
}

fn binary_partition(mut min: i32, mut max: i32, low: char, high: char, command: &str) -> i32 {
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

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Seat {
    row: i32,
    col: i32,
}

impl Seat {
    fn id(&self) -> i32 {
        self.row * 8 + self.col
    }
}
