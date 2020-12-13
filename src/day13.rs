use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./input/input13.txt")?;
    let mut lines = input.lines();

    let start_time = lines.next().expect("time").parse::<u64>()?;
    let bus_input = lines.next().expect("bus input");

    let result = bus_input
        .split(',')
        .filter(|s| *s != "x")
        .map(|s| s.parse::<u64>().expect("number"))
        .map(|bus_id| (bus_id, bus_id - (start_time % bus_id)))
        .min_by_key(|(_, time)| *time)
        .expect("non-empty puzzle input");

    println!("Day 13a: {}", result.0 * result.1);

    Ok(())
}
