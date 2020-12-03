use std::error::Error;

mod day01;
mod day02;
mod day03;

fn main() -> Result<(), Box<dyn Error>> {
    day03::run()
}
