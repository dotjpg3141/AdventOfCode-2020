use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./input/input15.txt")?
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse())
        .collect::<Result<Vec<usize>, _>>()?;

    let mut spoken = input;
    while spoken.len() < 2020 {
        let number = *spoken.last().unwrap();
        let next = spoken[0..spoken.len() - 1]
            .iter()
            .copied()
            .enumerate()
            .rev()
            .find_map(|(idx, num)| if num == number { Some(idx) } else { None })
            .map(|idx| spoken.len() - idx - 1)
            .unwrap_or_default();

        spoken.push(next);
    }

    println!("Day 15a: {:?}", spoken.last());

    Ok(())
}
