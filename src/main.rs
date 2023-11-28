use std::env;

mod advent_of_code_2022;
mod read_file;

use crate::advent_of_code_2022::advent_of_code_2022;

fn main() {
    let (year, day) = get_year_and_day();

    println!("Starting {}-{}", year, day);

    match year {
        2022 => advent_of_code_2022(day),
        2023 => advent_of_code_2023(day),
        _ => panic!("Unexpected year {}", year),
    }
}

fn advent_of_code_2023(day: u8) {
    todo!()
}

fn get_year_and_day() -> (u16, u8) {
    const USAGE: &'static str = "Usage: YEAR DAY";
    const FORMAT: &'static str = "Expected a number";
    let mut args = env::args();

    let year: u16 = args.nth(1).expect(USAGE).parse().expect(FORMAT);
    let day: u8 = args.nth(0).expect(USAGE).parse().expect(FORMAT);

    (year, day)
}
