use crate::aoc_result::AOCResult;

mod day1;
mod day2;

pub fn advent_of_code_2023(day: u8) -> AOCResult {
    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        _ => todo!(),
    }
}
