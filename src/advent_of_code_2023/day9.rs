use crate::{aoc_result::AOCResult, read_file::read_file};

use self::parse::parse;

pub fn solve() -> AOCResult {
    let input = read_file(2023, 9).unwrap();
    let parsed = parse(&input);

    (silver(&parsed), gold(&parsed)).into()
}

fn silver(input: &[Line]) -> i32 {
    input.iter().map(|line| silver_line(line.as_ref())).sum()
}

fn silver_line(input: &[i32]) -> i32 {
    if input.iter().all(|e| *e == 0) {
        0
    } else {
        let derived = derive(input);
        input.last().unwrap() + silver_line(&derived)
    }
}

fn gold(input: &[Line]) -> i32 {
    input.iter().map(|line| gold_line(line.as_ref())).sum()
}

fn gold_line(input: &[i32]) -> i32 {
    if input.iter().all(|e| *e == 0) {
        0
    } else {
        let derived = derive(input);
        input.first().unwrap() - gold_line(&derived)
    }
}

fn derive(input: &[i32]) -> Vec<i32> {
    let mut iter = input.iter();
    let mut previous = *iter.next().unwrap();
    let mut result = Vec::new();

    for elem in iter {
        result.push(elem - previous);
        previous = *elem;
    }

    result
}

type Line = Vec<i32>;

mod parse {
    use super::Line;
    use nom::IResult;

    pub fn parse(input: &str) -> Vec<Line> {
        parse_internal(input).unwrap().1
    }

    fn parse_internal(input: &str) -> IResult<&str, Vec<Line>> {
        let (input, lines) =
            nom::multi::separated_list1(nom::character::complete::newline, line)(input)?;

        Ok((input, lines))
    }

    fn line(input: &str) -> IResult<&str, Line> {
        let (input, line) = nom::multi::separated_list1(
            nom::bytes::complete::tag(" "),
            nom::character::complete::i32,
        )(input)?;

        Ok((input, line))
    }
}

#[cfg(test)]
mod test {
    use super::{gold, parse::parse, silver};

    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_parse() {
        let parsed = parse(INPUT);

        assert_eq!(
            parsed,
            [
                [0, 3, 6, 9, 12, 15],
                [1, 3, 6, 10, 15, 21],
                [10, 13, 16, 21, 30, 45]
            ]
        );
    }

    #[test]
    fn test_silver() {
        let parsed = parse(INPUT);

        assert_eq!(silver(&parsed), 114);
    }

    #[test]
    fn test_gold() {
        let parsed = parse(INPUT);

        assert_eq!(gold(&parsed), 2);
    }
}
