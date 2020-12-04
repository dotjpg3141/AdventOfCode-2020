use std::{error::Error, fs::File, io::BufRead, io::BufReader};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input04.txt")?;
    let reader = BufReader::new(file);

    let mut passport = Passport::default();
    let mut valid_count = 0;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            if passport.is_valid() {
                valid_count += 1;
            }
            passport = Passport::default();
        }

        let mut split = line
            .split(|c| c == ':' || c == ' ')
            .filter(|s| !s.is_empty());
        while let Some(name) = split.next() {
            let item = match name {
                "byr" => &mut passport.byr,
                "iyr" => &mut passport.iyr,
                "eyr" => &mut passport.eyr,
                "hgt" => &mut passport.hgt,
                "hcl" => &mut passport.hcl,
                "ecl" => &mut passport.ecl,
                "pid" => &mut passport.pid,
                "cid" => &mut passport.cid,
                other => {
                    eprintln!("Unexpected item: {} = {:?}", other, split.next());
                    continue;
                }
            };

            let value = split.next().expect("missing value");
            *item = Some(value.to_string());
        }
    }

    if passport.is_valid() {
        valid_count += 1;
    }

    println!("Day 4a: {}", valid_count);

    Ok(())
}

#[derive(Default)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }
}
