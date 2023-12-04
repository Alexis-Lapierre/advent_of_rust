use std::collections::HashSet;

use crate::aoc_result::AOCResult;

pub fn solve() -> AOCResult {
    (0, 0).into()
}

fn silver_line(line: &Line) -> u32 {
    let winning_count = u32::try_from(line.lucky.intersection(&line.winning).count()).unwrap();

    match winning_count {
        0 => 0,
        _ => 2_u32.pow(winning_count - 1),
    }
}

#[derive(Debug, PartialEq)]
struct Line {
    card: u8,
    lucky: HashSet<u8>,
    winning: HashSet<u8>,
}

mod parse {
    use std::collections::HashSet;

    use super::Line;

    pub fn parse(input: &str) -> impl Iterator<Item = Line> + '_ {
        input.lines().map(line)
    }

    fn line(input: &str) -> Line {
        line_internal(input).unwrap().1
    }

    fn line_internal(input: &str) -> nom::IResult<&str, Line> {
        let (input, card) = nom::sequence::delimited(
            nom::bytes::complete::tag("Card "),
            nom::character::complete::u8,
            nom::sequence::tuple((
                nom::bytes::complete::tag(":"),
                nom::character::complete::space1,
            )),
        )(input)?;

        let (input, (lucky, winning)) = nom::sequence::separated_pair(
            space_separated_numbers,
            nom::sequence::tuple((
                nom::character::complete::space0,
                nom::bytes::complete::tag("|"),
                nom::character::complete::space0,
            )),
            space_separated_numbers,
        )(input)?;

        Ok((
            input,
            Line {
                card,
                lucky: HashSet::from_iter(lucky.iter().cloned()),
                winning: HashSet::from_iter(winning.iter().cloned()),
            },
        ))
    }

    fn space_separated_numbers(input: &str) -> nom::IResult<&str, Vec<u8>> {
        nom::multi::separated_list1(
            nom::character::complete::space1,
            nom::character::complete::u8,
        )(input)
    }
}

#[cfg(test)]
mod test {
    use super::{parse::parse, silver_line};
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_parse() {
        let mut lines = parse(INPUT);
        assert_eq!(silver_line(&lines.next().unwrap()), 8);
        assert_eq!(silver_line(&lines.next().unwrap()), 2);
        assert_eq!(silver_line(&lines.next().unwrap()), 2);
        assert_eq!(silver_line(&lines.next().unwrap()), 1);
        assert_eq!(silver_line(&lines.next().unwrap()), 0);
        assert_eq!(silver_line(&lines.next().unwrap()), 0);
        assert_eq!(lines.next(), None);
    }
}
