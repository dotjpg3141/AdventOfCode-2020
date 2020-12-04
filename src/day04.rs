use std::{error::Error, fs::File, io::BufRead, io::BufReader};

pub fn run() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input04.txt")?;
    let reader = BufReader::new(file);

    let mut passport = Passport::default();
    let mut valid_a_count = 0;
    let mut valid_b_count = 0;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            if passport.is_valid_a() {
                valid_a_count += 1;
            }
            if passport.is_valid_b() {
                valid_b_count += 1;
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

    if passport.is_valid_a() {
        valid_a_count += 1;
    }

    if passport.is_valid_b() {
        valid_b_count += 1;
    }

    println!("Day 4a: {}", valid_a_count);
    println!("Day 4b: {}", valid_b_count);

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
    fn is_valid_a(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid_b(&self) -> bool {
        validate_number(self.byr.as_ref(), 4, 1920, 2002)
            && validate_number(self.iyr.as_ref(), 4, 2010, 2020)
            && validate_number(self.eyr.as_ref(), 4, 2020, 2030)
            && validate_height(self.hgt.as_ref())
            && validate_hex_color(self.hcl.as_ref())
            && validate_eye_color(self.ecl.as_ref())
            && validate_number(self.pid.as_ref(), 9, 0, i32::MAX)
    }
}

fn validate_number(item: Option<&String>, len: usize, min: i32, max: i32) -> bool {
    let item = if let Some(item) = item {
        item
    } else {
        return false;
    };

    if item.len() != len {
        return false;
    }

    let num = match item.parse::<i32>() {
        Ok(num) => num,
        Err(_) => return false,
    };

    min <= num && num <= max
}

fn validate_height(item: Option<&String>) -> bool {
    let item = if let Some(item) = item {
        item
    } else {
        return false;
    };

    if item.len() < 3 {
        return false;
    }

    let (num, unit) = item.split_at(item.len() - 2); // NOTE(jpg): asuming ascii input only

    let num = match num.parse::<i32>() {
        Ok(num) => num,
        Err(_) => return false,
    };

    match unit {
        "cm" => 150 <= num && num <= 193,
        "in" => 59 <= num && num <= 76,
        _ => false,
    }
}

fn validate_hex_color(item: Option<&String>) -> bool {
    let item = if let Some(item) = item {
        item
    } else {
        return false;
    };

    if item.len() != 7 || item.chars().next() != Some('#') {
        return false;
    }

    item.chars().skip(1).all(|c| c.is_ascii_hexdigit())
}

fn validate_eye_color(item: Option<&String>) -> bool {
    let item = if let Some(item) = item {
        item
    } else {
        return false;
    };

    match item.as_str() {
        "amb" => true,
        "blu" => true,
        "brn" => true,
        "gry" => true,
        "grn" => true,
        "hzl" => true,
        "oth" => true,
        _ => false,
    }
}
