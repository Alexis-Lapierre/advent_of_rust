mod day4;
pub fn advent_of_code_2022(day: u8) {
    let result = match day {
        4 => day4::solve(),
        _ => todo!(),
    };

    println!("{result:?}");
}
