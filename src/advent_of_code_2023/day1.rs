use crate::read_file::read_file;

pub fn solve() -> (u32, u32) {
    let content = read_file(2023, 1).expect("File input/2023/01.txt to exist");
    let silver_parsed = parse::lines(&content);
    let gold_parsed = parse::gold_lines(&content);

    (silver(silver_parsed), gold(gold_parsed))
}

fn silver(elems: impl Iterator<Item = u32>) -> u32 {
    elems.sum()
}

fn gold(lines: impl Iterator<Item = Vec<u8>>) -> u32 {
    lines.map(|line| gold_line(&line)).sum()
}

fn gold_line(line: &[u8]) -> u32 {
    // First * 10 + Last
    u32::from(*line.first().unwrap() * 10 + *line.last().unwrap())
}

mod parse {
    use nom::{
        bytes::complete::{tag, take},
        character::complete::alpha0,
    };

    fn line(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
        let first = first_number(input)?;
        let rev_input = input.chars().rev().collect::<String>();
        let last = first_number(rev_input.as_str()).unwrap();

        Ok(first * 10 + last)
    }

    fn first_number(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
        let (input, _) = alpha0(input)?;
        let searched = input.chars().next().unwrap().to_digit(10).unwrap();
        Ok(searched)
    }

    pub fn lines(input: &str) -> impl Iterator<Item = u32> + '_ {
        input.lines().map(line).map(std::result::Result::unwrap)
    }

    fn gold_line(input: &str) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let mut previous_input = input;
        while let Ok((input, char)) = gold_selector(previous_input) {
            if let Some(n) = match char {
                "one" | "1" => Some(1),
                "two" | "2" => Some(2),
                "three" | "3" => Some(3),
                "four" | "4" => Some(4),
                "five" | "5" => Some(5),
                "six" | "6" => Some(6),
                "seven" | "7" => Some(7),
                "eight" | "8" => Some(8),
                "nine" | "9" => Some(9),
                _ => None,
            } {
                result.push(n);
            }

            let (the_previous_input, _) = take_one(previous_input).unwrap();
            previous_input = the_previous_input;
        }

        result
    }

    fn gold_selector(input: &str) -> Result<(&str, &str), nom::Err<nom::error::Error<&str>>> {
        nom::branch::alt((
            tag("one"),
            tag("two"),
            tag("three"),
            tag("four"),
            tag("five"),
            tag("six"),
            tag("seven"),
            tag("eight"),
            tag("nine"),
            take(1usize),
        ))(input)
    }

    fn take_one(input: &str) -> Result<(&str, &str), nom::Err<nom::error::Error<&str>>> {
        take(1usize)(input)
    }

    pub fn gold_lines(input: &str) -> impl Iterator<Item = Vec<u8>> + '_ {
        input.lines().map(gold_line)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const SILVER_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const GOLD_INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test_parse() {
        let lines = parse::lines(SILVER_INPUT).collect::<Vec<u32>>();
        assert_eq!(lines, vec![12, 38, 15, 77,]);
    }

    #[test]
    fn test_gold_parse() {
        let lines = parse::gold_lines(SILVER_INPUT)
            .flatten()
            .collect::<Vec<u8>>();
        assert_eq!(lines, vec![1, 2, 3, 8, 1, 2, 3, 4, 5, 7]);

        let lines = parse::gold_lines(GOLD_INPUT).flatten().collect::<Vec<u8>>();

        assert_eq!(
            lines,
            vec![
                2, 1, 9, //
                8, 2, 3, //
                1, 2, 3, //
                2, 1, 3, 4, //
                4, 9, 8, 7, 2, 1, 8, 2, 3, 4, //
                7, 6
            ]
        );
    }

    #[test]
    fn test_gold() {
        let lines = parse::gold_lines(SILVER_INPUT);
        assert_eq!(gold(lines), 142);
        let lines = parse::gold_lines(GOLD_INPUT);
        assert_eq!(gold(lines), 281);
    }
}
