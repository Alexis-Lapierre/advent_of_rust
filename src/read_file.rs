pub fn read_file(year: u16, day: u8) -> Result<String, std::io::Error> {
    std::fs::read_to_string(format!("./input/{}/{:02}.txt", year, day))
}
