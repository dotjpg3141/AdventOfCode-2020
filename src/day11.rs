use std::error::Error;

use crate::parse_lines;

const NEIGHBOURS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut seats = parse_lines(11, |line| line.chars().map(Seat::from).collect::<Vec<_>>())?;

    loop {
        let next = step(&seats);
        if seats == next {
            break;
        }
        seats = next;
    }

    let occupied_seats = seats
        .iter()
        .flatten()
        .copied()
        .filter(|&s| s == Seat::Occupied)
        .count();

    println!("Day 11a: {}", occupied_seats);

    Ok(())
}

fn step(seats: &[Vec<Seat>]) -> Vec<Vec<Seat>> {
    let height = seats.len();
    let mut target = Vec::with_capacity(seats.len());

    for (row_idx, row) in seats.iter().enumerate() {
        let width = row.len();
        let mut target_row = Vec::with_capacity(width);

        for (col_idx, seat) in row.iter().copied().enumerate() {
            let opccupied_neighbours = NEIGHBOURS
                .iter()
                .copied()
                .map(|(dx, dy)| (col_idx as i32 + dx, row_idx as i32 + dy))
                .filter(|(x, y)| (0..width as i32).contains(x) && (0..height as i32).contains(y))
                .map(|(x, y)| seats[y as usize][x as usize])
                .filter(|&seat| seat == Seat::Occupied)
                .count();

            let target_seat = match seat {
                Seat::Floor => Seat::Floor,
                Seat::Empty => {
                    if opccupied_neighbours == 0 {
                        Seat::Occupied
                    } else {
                        Seat::Empty
                    }
                }
                Seat::Occupied => {
                    if opccupied_neighbours >= 4 {
                        Seat::Empty
                    } else {
                        Seat::Occupied
                    }
                }
            };

            target_row.push(target_seat);
        }

        target.push(target_row);
    }

    target
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for Seat {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            other => panic!("invalid seat: '{}'", other),
        }
    }
}
