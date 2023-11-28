use std::env;

fn main() {
    let day = get_day(&env::args().nth(1));

    println!("Hello, world! Your day is {}", day);
}

fn get_day(maybe_day: &Option<String>) -> u8 {
    if let Some(day_str) = maybe_day {
        day_str
            .parse()
            .expect("First given argument to be a string")
    } else {
        println!("No day given, defaulting to first day");
        1
    }
}
