use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./input/input17.txt")?;
    v1::run(&input)?;
    v2::run(&input)?;
    Ok(())
}

mod v1 {
    use std::{
        collections::{HashMap, HashSet},
        error::Error,
    };

    const NEIGHBOURS: [(i32, i32, i32); 26] = [
        (-1, -1, -1),
        (-1, -1, 0),
        (-1, -1, 1),
        (-1, 0, -1),
        (-1, 0, 0),
        (-1, 0, 1),
        (-1, 1, -1),
        (-1, 1, 0),
        (-1, 1, 1),
        (0, -1, -1),
        (0, -1, 0),
        (0, -1, 1),
        (0, 0, -1),
        (0, 0, 1),
        (0, 1, -1),
        (0, 1, 0),
        (0, 1, 1),
        (1, -1, -1),
        (1, -1, 0),
        (1, -1, 1),
        (1, 0, -1),
        (1, 0, 0),
        (1, 0, 1),
        (1, 1, -1),
        (1, 1, 0),
        (1, 1, 1),
    ];

    pub fn run(input: &str) -> Result<(), Box<dyn Error>> {
        let mut map = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                let is_active = symbol == '#';
                if is_active {
                    map.insert((x as i32, y as i32, 0));
                }
            }
        }

        for _ in 0..6 {
            map = step(&map);
        }

        println!("Day 17a: {}", map.len());

        Ok(())
    }

    fn step(input: &HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
        let mut next = HashSet::new();

        let active = input.iter().copied();
        for active_cell in active {
            let active_neighbours = get_active_neighours(input, active_cell).count();
            if active_neighbours == 2 || active_neighbours == 3 {
                next.insert(active_cell);
            }
        }

        let inactive = input
            .iter()
            .copied()
            .flat_map(|coord| get_inactive_neighours(input, coord));

        let mut inactive_counter: HashMap<_, usize> = HashMap::new();
        for inactive_cell in inactive {
            *inactive_counter.entry(inactive_cell).or_default() += 1;
        }

        for (inactive_cell, active_neighbours) in inactive_counter {
            if active_neighbours == 3 {
                next.insert(inactive_cell);
            }
        }

        next
    }

    fn get_neighbours(coord: (i32, i32, i32)) -> impl Iterator<Item = (i32, i32, i32)> {
        let (x, y, z) = coord;
        NEIGHBOURS
            .iter()
            .copied()
            .map(move |(dx, dy, dz)| (x + dx, y + dy, z + dz))
    }

    fn get_active_neighours<'a>(
        input: &'a HashSet<(i32, i32, i32)>,
        coord: (i32, i32, i32),
    ) -> impl 'a + Iterator<Item = (i32, i32, i32)> {
        get_neighbours(coord).filter(move |(x, y, z)| input.contains(&(*x, *y, *z)))
    }

    fn get_inactive_neighours<'a>(
        input: &'a HashSet<(i32, i32, i32)>,
        coord: (i32, i32, i32),
    ) -> impl 'a + Iterator<Item = (i32, i32, i32)> {
        get_neighbours(coord).filter(move |(x, y, z)| !input.contains(&(*x, *y, *z)))
    }
}

mod v2 {
    use std::{
        collections::{HashMap, HashSet},
        error::Error,
    };

    const NEIGHBOURS: [(i32, i32, i32, i32); 80] = [
        (-1, -1, -1, -1),
        (-1, -1, -1, 0),
        (-1, -1, -1, 1),
        (-1, -1, 0, -1),
        (-1, -1, 0, 0),
        (-1, -1, 0, 1),
        (-1, -1, 1, -1),
        (-1, -1, 1, 0),
        (-1, -1, 1, 1),
        (-1, 0, -1, -1),
        (-1, 0, -1, 0),
        (-1, 0, -1, 1),
        (-1, 0, 0, -1),
        (-1, 0, 0, 0),
        (-1, 0, 0, 1),
        (-1, 0, 1, -1),
        (-1, 0, 1, 0),
        (-1, 0, 1, 1),
        (-1, 1, -1, -1),
        (-1, 1, -1, 0),
        (-1, 1, -1, 1),
        (-1, 1, 0, -1),
        (-1, 1, 0, 0),
        (-1, 1, 0, 1),
        (-1, 1, 1, -1),
        (-1, 1, 1, 0),
        (-1, 1, 1, 1),
        (0, -1, -1, -1),
        (0, -1, -1, 0),
        (0, -1, -1, 1),
        (0, -1, 0, -1),
        (0, -1, 0, 0),
        (0, -1, 0, 1),
        (0, -1, 1, -1),
        (0, -1, 1, 0),
        (0, -1, 1, 1),
        (0, 0, -1, -1),
        (0, 0, -1, 0),
        (0, 0, -1, 1),
        (0, 0, 0, -1),
        (0, 0, 0, 1),
        (0, 0, 1, -1),
        (0, 0, 1, 0),
        (0, 0, 1, 1),
        (0, 1, -1, -1),
        (0, 1, -1, 0),
        (0, 1, -1, 1),
        (0, 1, 0, -1),
        (0, 1, 0, 0),
        (0, 1, 0, 1),
        (0, 1, 1, -1),
        (0, 1, 1, 0),
        (0, 1, 1, 1),
        (1, -1, -1, -1),
        (1, -1, -1, 0),
        (1, -1, -1, 1),
        (1, -1, 0, -1),
        (1, -1, 0, 0),
        (1, -1, 0, 1),
        (1, -1, 1, -1),
        (1, -1, 1, 0),
        (1, -1, 1, 1),
        (1, 0, -1, -1),
        (1, 0, -1, 0),
        (1, 0, -1, 1),
        (1, 0, 0, -1),
        (1, 0, 0, 0),
        (1, 0, 0, 1),
        (1, 0, 1, -1),
        (1, 0, 1, 0),
        (1, 0, 1, 1),
        (1, 1, -1, -1),
        (1, 1, -1, 0),
        (1, 1, -1, 1),
        (1, 1, 0, -1),
        (1, 1, 0, 0),
        (1, 1, 0, 1),
        (1, 1, 1, -1),
        (1, 1, 1, 0),
        (1, 1, 1, 1),
    ];

    pub fn run(input: &str) -> Result<(), Box<dyn Error>> {
        let mut map = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, symbol) in line.chars().enumerate() {
                let is_active = symbol == '#';
                if is_active {
                    map.insert((x as i32, y as i32, 0, 0));
                }
            }
        }

        for _ in 0..6 {
            map = step(&map);
        }

        println!("Day 17a: {}", map.len());

        Ok(())
    }

    fn step(input: &HashSet<(i32, i32, i32, i32)>) -> HashSet<(i32, i32, i32, i32)> {
        let mut next = HashSet::new();

        let active = input.iter().copied();
        for active_cell in active {
            let active_neighbours = get_active_neighours(input, active_cell).count();
            if active_neighbours == 2 || active_neighbours == 3 {
                next.insert(active_cell);
            }
        }

        let inactive = input
            .iter()
            .copied()
            .flat_map(|coord| get_inactive_neighours(input, coord));

        let mut inactive_counter: HashMap<_, usize> = HashMap::new();
        for inactive_cell in inactive {
            *inactive_counter.entry(inactive_cell).or_default() += 1;
        }

        for (inactive_cell, active_neighbours) in inactive_counter {
            if active_neighbours == 3 {
                next.insert(inactive_cell);
            }
        }

        next
    }

    fn get_neighbours(coord: (i32, i32, i32, i32)) -> impl Iterator<Item = (i32, i32, i32, i32)> {
        let (x, y, z, w) = coord;
        NEIGHBOURS
            .iter()
            .copied()
            .map(move |(dx, dy, dz, dw)| (x + dx, y + dy, z + dz, w + dw))
    }

    fn get_active_neighours<'a>(
        input: &'a HashSet<(i32, i32, i32, i32)>,
        coord: (i32, i32, i32, i32),
    ) -> impl 'a + Iterator<Item = (i32, i32, i32, i32)> {
        get_neighbours(coord).filter(move |(x, y, z, w)| input.contains(&(*x, *y, *z, *w)))
    }

    fn get_inactive_neighours<'a>(
        input: &'a HashSet<(i32, i32, i32, i32)>,
        coord: (i32, i32, i32, i32),
    ) -> impl 'a + Iterator<Item = (i32, i32, i32, i32)> {
        get_neighbours(coord).filter(move |(x, y, z, w)| !input.contains(&(*x, *y, *z, *w)))
    }
}
