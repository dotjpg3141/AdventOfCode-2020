use std::{error::Error, iter::repeat, time::Instant};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

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
        day08::run,
        day09::run,
        day10::run,
        day11::run,
        day12::run,
    ];

    match args.as_slice() {
        [] => days.last().unwrap()(),
        ["all"] => {
            for day in &days {
                day()?;
            }
            Ok(())
        }
        ["perf"] => {
            let mut performance = Vec::new();
            for day in &days {
                let start = Instant::now();
                day()?;
                performance.push(start.elapsed().as_millis());
            }
            println!();

            let max_duration = performance.iter().copied().max().expect("non-empty").max(1);
            let duration_per_char = 80.0 / max_duration as f64;
            for (idx, duration) in performance.iter().enumerate() {
                let char_count = ((*duration as f64) * duration_per_char) as usize;
                let marker = repeat('+').take(char_count).collect::<String>();
                println!("Day {:>2 }: {}", idx + 1, marker);
            }
            println!();

            for (idx, duration) in performance.iter().enumerate() {
                println!("Day {:>2 }: {:>6} ms", idx + 1, duration);
            }
            println!();

            println!("Worst performance");
            let mut worst_peformance = performance.iter().enumerate().collect::<Vec<_>>();
            worst_peformance.sort_by_key(|(_, duration)| -(**duration as i64));
            for (idx, duration) in worst_peformance.into_iter().take(3) {
                println!("Day {:>2 }: {:>6} ms", idx + 1, duration);
            }
            println!();

            let total_duration = performance.iter().sum::<u128>();
            println!("Total: {:>6} ms", total_duration);

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
