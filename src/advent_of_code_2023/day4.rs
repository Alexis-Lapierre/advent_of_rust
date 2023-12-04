use std::collections::{HashSet, VecDeque};

use crate::{aoc_result::AOCResult, read_file::read_file};

pub fn solve() -> AOCResult {
    let input = read_file(2023, 4).expect("File input/2023/04.txt to exist");
    let parsed: Vec<_> = parse::parse(&input).collect();
    (silver(parsed.iter().cloned()), gold(parsed.iter().cloned())).into()
}

fn silver<'a>(lines: impl Iterator<Item = Line>) -> u32 {
    lines.map(silver_line).sum()
}

fn silver_line(line: Line) -> u32 {
    let winning_count = u32::from(line.winning_numbers_count);

    match winning_count {
        0 => 0,
        _ => 2_u32.pow(winning_count - 1),
    }
}

fn gold<'a>(lines: impl Iterator<Item = Line>) -> u32 {
    let number_of_cards = VecDeque::new();
    gold_recursive(lines, number_of_cards)
}

fn gold_recursive(mut lines: impl Iterator<Item = Line>, mut cards_queue: VecDeque<u32>) -> u32 {
    if let Some(line) = lines.next() {
        let current_cards = cards_queue.pop_front().unwrap_or(1);
        let wins = usize::from(line.winning_numbers_count);
        while cards_queue.len() < wins {
            cards_queue.push_back(1);
        }
        for card in cards_queue.range_mut(..wins) {
            *card += current_cards;
        }

        current_cards + gold_recursive(lines, cards_queue)
    } else {
        0
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Line {
    winning_numbers_count: u8,
}

#[derive(Debug, PartialEq)]
struct RawParsedLine {
    card: u8,
    lucky: HashSet<u8>,
    winning: HashSet<u8>,
}

impl From<RawParsedLine> for Line {
    fn from(line: RawParsedLine) -> Self {
        Self {
            winning_numbers_count: u8::try_from(line.lucky.intersection(&line.winning).count())
                .expect("Winning number should be less than u8::max"),
        }
    }
}

mod parse {
    use std::collections::HashSet;

    use nom::{bytes::complete::tag, character::complete::space1};

    use super::{Line, RawParsedLine};

    pub fn parse(input: &str) -> impl Iterator<Item = Line> + '_ {
        input.lines().map(line)
    }

    fn line(input: &str) -> Line {
        line_internal(input).unwrap().1
    }

    fn line_internal(input: &str) -> nom::IResult<&str, Line> {
        let (input, card) = nom::sequence::delimited(
            nom::sequence::tuple((tag("Card"), space1)),
            nom::character::complete::u8,
            nom::sequence::tuple((tag(":"), space1)),
        )(input)?;

        let (input, (lucky, winning)) = nom::sequence::separated_pair(
            space_separated_numbers,
            nom::sequence::tuple((space1, tag("|"), space1)),
            space_separated_numbers,
        )(input)?;

        Ok((
            input,
            RawParsedLine {
                card,
                lucky: HashSet::from_iter(lucky.iter().cloned()),
                winning: HashSet::from_iter(winning.iter().cloned()),
            }
            .into(),
        ))
    }

    fn space_separated_numbers(input: &str) -> nom::IResult<&str, Vec<u8>> {
        nom::multi::separated_list1(space1, nom::character::complete::u8)(input)
    }
}

#[cfg(test)]
mod test {
    use super::{gold, parse::parse, silver, silver_line};
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_silver_line() {
        let mut lines = parse(INPUT);
        assert_eq!(silver_line(lines.next().unwrap()), 8);
        assert_eq!(silver_line(lines.next().unwrap()), 2);
        assert_eq!(silver_line(lines.next().unwrap()), 2);
        assert_eq!(silver_line(lines.next().unwrap()), 1);
        assert_eq!(silver_line(lines.next().unwrap()), 0);
        assert_eq!(silver_line(lines.next().unwrap()), 0);
        assert_eq!(lines.next(), None);
    }

    #[test]
    fn test_silver() {
        let lines = parse(INPUT);
        assert_eq!(silver(lines), 13);
    }

    #[test]
    fn test_gold() {
        let lines = parse(INPUT);
        assert_eq!(gold(lines), 30);
    }
}
