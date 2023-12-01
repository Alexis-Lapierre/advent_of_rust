mod day1;
pub fn advent_of_code_2023(day: u8) {
    let result = match day {
        1 => day1::solve(),
        _ => todo!(),
    };

    println!("{result:?}");
}
