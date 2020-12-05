use std::error::Error;

use crate::parse_lines;

pub fn run() -> Result<(), Box<dyn Error>> {
    let map = parse_lines(3, |line| line.chars().map(|c| c == '#').collect::<Vec<_>>())?;

    let height = map.len();
    let width = map[0].len();

    let has_tree = |x: usize, y: usize| {
        let x = x % width;
        map[y][x]
    };

    let calculate_slope = |dx, dy| {
        let mut x = 0;
        let mut y = 0;
        let mut tree_count = 0u64;

        while y < height {
            if has_tree(x, y) {
                tree_count += 1;
            }

            x += dx;
            y += dy;
        }

        tree_count
    };

    debug_assert!(!has_tree(0, 0));
    let tree_count = calculate_slope(3, 1);
    println!("Day 3a: {}", tree_count);

    let result = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(dx, dy)| calculate_slope(*dx, *dy))
        .product::<u64>();
    println!("Day 3b: {}", result);

    Ok(())
}
