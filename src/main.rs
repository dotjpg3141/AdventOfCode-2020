use std::error::Error;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    let days = [day01::run, day02::run, day03::run, day04::run, day05::run];

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
