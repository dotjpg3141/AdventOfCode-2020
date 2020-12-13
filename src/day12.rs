use std::error::Error;

use crate::parse_lines;

pub fn run() -> Result<(), Box<dyn Error>> {
    let actions = parse_lines(12, |s| parse_action(&s))?;

    let mut ship = Ship {
        direction: Direction::East,
        x: 0,
        y: 0,
    };

    for action in actions.iter().copied() {
        ship.eval(action);
    }

    let result = ship.x + ship.y;
    println!("Day 12a: {}", result);

    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Move(Direction, i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Direction {
    East,
    North,
    West,
    South,
}

struct Ship {
    direction: Direction,
    x: i32,
    y: i32,
}

impl Ship {
    fn eval(&mut self, action: Action) {
        match action {
            Action::Move(direction, distance) => self.move_in_direction(direction, distance),
            Action::Left(angle) => self.turn(angle),
            Action::Right(angle) => self.turn(-angle),
            Action::Forward(distance) => self.move_in_direction(self.direction, distance),
        }
    }

    fn move_in_direction(&mut self, dir: Direction, distance: i32) {
        let (dx, dy) = match dir {
            Direction::East => (1, 0),
            Direction::North => (0, -1),
            Direction::West => (-1, 0),
            Direction::South => (0, 1),
        };

        self.x += dx * distance;
        self.y += dy * distance;
    }

    fn turn(&mut self, angle: i32) {
        debug_assert_eq!(angle % 90, 0);

        let direction = (self.direction as i32 + (angle / 90)).rem_euclid(4);
        self.direction = unsafe { std::mem::transmute(direction as u8) };
    }
}

fn parse_action(s: &str) -> Action {
    let mut s = s.chars();
    let opcode = s.next().expect("opcode");
    let num = s.as_str().parse().expect("number");

    match opcode {
        'W' => Action::Move(Direction::West, num),
        'N' => Action::Move(Direction::North, num),
        'E' => Action::Move(Direction::East, num),
        'S' => Action::Move(Direction::South, num),
        'L' => Action::Left(num),
        'R' => Action::Right(num),
        'F' => Action::Forward(num),
        other => panic!("invalid opcode: {}", other),
    }
}
