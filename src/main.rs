use std::error::Error;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let days = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
    ];

    match args.as_slice() {
        [] => days.last().unwrap()(),
        ["all"] => {
            for day in &days {
                day()?;
            }
            Ok(())
        }
        _ => {
            panic!("Unrecognized arguments: {:?}", args)
        }
    }
}

fn try_parse_lines<T, E>(
    day: u32,
    f: impl Fn(String) -> Result<T, E>,
) -> Result<Vec<T>, Box<dyn Error>>
where
    E: Into<Box<dyn Error>>,
{
    use std::{fs::File, io::BufRead, io::BufReader};

    let path = format!("./input/input{:0>2}.txt", day);

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let entries = reader
        .lines()
        .map(|line| -> Result<_, Box<dyn Error>> { Ok(f(line?).map_err(|e| e.into())?) })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(entries)
}

fn parse_lines<T>(day: u32, f: impl Fn(String) -> T) -> Result<Vec<T>, Box<dyn Error>> {
    try_parse_lines(day, |line| -> Result<T, std::convert::Infallible> {
        Ok(f(line))
    })
}
