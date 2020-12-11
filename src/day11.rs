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
    let seats = parse_lines(11, |line| line.chars().map(Seat::from).collect::<Vec<_>>())?;

    let occupied_seats = simulate(&seats, direct_neighbours);
    println!("Day 11a: {}", occupied_seats);

    let occupied_seats = simulate(&seats, neighbours_in_sight);
    println!("Day 11b: {}", occupied_seats);

    Ok(())
}

fn direct_neighbours(seats: &[Vec<Seat>], col_idx: usize, row_idx: usize) -> Seat {
    let height = seats.len();
    let width = seats[0].len();

    let occupied = NEIGHBOURS
        .iter()
        .copied()
        .map(|(dx, dy)| (col_idx as i32 + dx, row_idx as i32 + dy))
        .filter(|(x, y)| (0..width as i32).contains(x) && (0..height as i32).contains(y))
        .map(|(x, y)| seats[y as usize][x as usize])
        .filter(|&seat| seat == Seat::Occupied)
        .count();

    let seat = seats[row_idx][col_idx];
    match seat {
        Seat::Floor => Seat::Floor,
        Seat::Empty => {
            if occupied == 0 {
                Seat::Occupied
            } else {
                Seat::Empty
            }
        }
        Seat::Occupied => {
            if occupied >= 4 {
                Seat::Empty
            } else {
                Seat::Occupied
            }
        }
    }
}

fn neighbours_in_sight(seats: &[Vec<Seat>], col_idx: usize, row_idx: usize) -> Seat {
    let occupied = NEIGHBOURS
        .iter()
        .copied()
        .map(|(dx, dy)| neighbours_ray_cast(seats, col_idx, row_idx, dx, dy) as usize)
        .sum::<usize>();

    let seat = seats[row_idx][col_idx];
    match seat {
        Seat::Floor => Seat::Floor,
        Seat::Empty => {
            if occupied == 0 {
                Seat::Occupied
            } else {
                Seat::Empty
            }
        }
        Seat::Occupied => {
            if occupied >= 5 {
                Seat::Empty
            } else {
                Seat::Occupied
            }
        }
    }
}

fn neighbours_ray_cast(seats: &[Vec<Seat>], x: usize, y: usize, dx: i32, dy: i32) -> bool {
    let mut x = x as i32;
    let mut y = y as i32;

    loop {
        x += dx;
        y += dy;

        let seat = seats
            .get(y as usize)
            .map(|row| row.get(x as usize))
            .flatten();

        match seat {
            None => return false,
            Some(Seat::Floor) => {}
            Some(Seat::Empty) => return false,
            Some(Seat::Occupied) => return true,
        }
    }
}

fn step(
    seats: &[Vec<Seat>],
    get_next_seat: &impl Fn(&[Vec<Seat>], usize, usize) -> Seat,
) -> Vec<Vec<Seat>> {
    let mut target = Vec::with_capacity(seats.len());

    for (row_idx, row) in seats.iter().enumerate() {
        let width = row.len();
        let mut target_row = Vec::with_capacity(width);
        for col_idx in 0..width {
            let target_seat = get_next_seat(seats, col_idx, row_idx);

            target_row.push(target_seat);
        }

        target.push(target_row);
    }

    target
}

fn simulate(
    seats: &[Vec<Seat>],
    get_next_seat: impl Fn(&[Vec<Seat>], usize, usize) -> Seat,
) -> usize {
    let mut seats = step(seats, &get_next_seat);

    loop {
        let next = step(&seats, &get_next_seat);
        if seats == next {
            break;
        }
        seats = next;
    }

    seats
        .iter()
        .flatten()
        .copied()
        .filter(|&s| s == Seat::Occupied)
        .count()
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
