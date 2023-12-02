use crate::aoc_result::AOCResult;

mod day4;
pub fn advent_of_code_2022(day: u8) -> AOCResult {
    match day {
        4 => day4::solve(),
        _ => todo!(),
    }
}
