use std::error::Error;

mod day01;
mod day02;

fn main() -> Result<(), Box<dyn Error>> {
    day02::run()
}
