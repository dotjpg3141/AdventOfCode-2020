use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./input/input13.txt")?;
    let mut lines = input.lines();

    let start_time = lines.next().expect("time").parse::<i64>()?;
    let bus_input = lines.next().expect("bus input");

    let result = bus_input
        .split(',')
        .filter(|s| *s != "x")
        .map(|s| s.parse::<i64>().expect("number"))
        .map(|bus_id| (bus_id, bus_id - (start_time % bus_id)))
        .min_by_key(|(_, time)| *time)
        .expect("non-empty puzzle input");

    println!("Day 13a: {}", result.0 * result.1);

    let result = bus_input
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(offset, s)| (offset as i64, s.parse::<i64>().expect("number")))
        .map(|(offset, id)| (id, (-offset).rem_euclid(id)))
        .fold((1, 0), solve)
        .1;

    println!("Day 13b: {}", result);

    Ok(())
}

fn solve(v1: (i64, i64), v2: (i64, i64)) -> (i64, i64) {
    // a_i = input[i].0
    // b_i = input[i].1
    // x % a_i == b_i
    let (a1, b1) = v1;
    let (a2, b2) = v2;

    let mut x = b1;
    loop {
        if x % a2 == b2 {
            return (a1 * a2, x);
        }

        x += a1;
    }
}
