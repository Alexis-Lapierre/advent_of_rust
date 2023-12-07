use itertools::Itertools;
use std::{cmp::Ordering, collections::HashMap};

use crate::{aoc_result::AOCResult, read_file::read_file};

pub fn solve() -> AOCResult {
    let input = read_file(2023, 7).expect("File input/2023/06.txt to exist");
    let mut parsed = parse::parse(&input);
    (silver(&mut parsed), gold(&mut parsed)).into()
}

fn silver(lines: &mut [Line]) -> u32 {
    lines.sort_unstable_by(|line, other| Hand::cmp(&line.hand.0, &other.hand.0));

    lines
        .iter()
        .zip(1..)
        .map(|(line, index)| line.bid * index)
        .sum()
}

fn gold(lines: &mut [Line]) -> u32 {
    lines.sort_unstable_by(|line, other| Hand::cmp(&line.hand.1, &other.hand.1));

    lines
        .iter()
        .zip(1..)
        .map(|(line, index)| line.bid * index)
        .sum()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
    Jocker,
}

impl TryFrom<u8> for Card {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'A' => Ok(Self::Ace),
            b'K' => Ok(Self::King),
            b'Q' => Ok(Self::Queen),
            b'J' => Ok(Self::Jack),
            b'T' => Ok(Self::Number(10)),
            b'2'..=b'9' => Ok(Self::Number(value - b'0')),
            _ => Err(format!("Invalid character '{value}' !")),
        }
    }
}

impl From<Card> for u8 {
    fn from(card: Card) -> Self {
        match card {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::Number(n) => n,
            Card::Jocker => 0,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        u8::cmp(&u8::from(*self), &u8::from(*other))
    }
}

impl Card {
    fn into_gold(self) -> Self {
        match self {
            Card::Jack => Card::Jocker,
            other => other,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl<T> From<T> for HandType
where
    T: Iterator<Item = Card>,
{
    fn from(cards: T) -> Self {
        let mut map: HashMap<Card, u8> = HashMap::new();
        for card in cards {
            map.entry(card).and_modify(|n| *n += 1).or_insert(1);
        }

        let jacks = map.remove(&Card::Jocker).unwrap_or(0);
        let mut iter = map.into_values().sorted().rev();
        let best = iter.next().unwrap_or(0) + jacks;
        let second_best = iter.next().unwrap_or(0);

        match (best, second_best) {
            (5, 0) => Self::FiveOfAKind,
            (4, 1) => Self::FourOfAKind,
            (3, 2) => Self::FullHouse,
            (3, 1) => Self::ThreeOfAKind,
            (2, 2) => Self::TwoPair,
            (2, 1) => Self::OnePair,
            (1, 1) => Self::HighCard,
            _ => panic!("Unexpected tuple: ({best}, {second_best})"),
        }
    }
}
impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        u8::cmp(&u8::from(*self), &u8::from(*other))
    }
}

impl From<HandType> for u8 {
    fn from(hand: HandType) -> Self {
        match hand {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
}

impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Self {
        Self {
            hand_type: HandType::from(cards.iter().copied()),
            cards,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => match self.cards.cmp(&other.cards) {
                Ordering::Equal => panic!("Two Hands are perfectly equal, should not append!"),
                ord => ord,
            },
            ord => ord,
        }
    }
}

#[derive(Debug)]
struct Line {
    hand: (Hand, Hand),
    bid: u32,
}

mod parse {
    use nom::IResult;

    use super::{Card, Line};
    pub fn parse(input: &str) -> Vec<Line> {
        internal(input).unwrap().1
    }

    fn internal(input: &str) -> IResult<&str, Vec<Line>> {
        nom::multi::separated_list1(nom::character::complete::newline, line)(input)
    }

    fn line(input: &str) -> IResult<&str, Line> {
        let (input, (cards, bid)) = nom::sequence::separated_pair(
            nom::character::complete::alphanumeric1,
            nom::bytes::complete::tag(" "),
            nom::character::complete::u32,
        )(input)?;

        let silver_cards = cards
            .as_bytes()
            .iter()
            .map(|card| Card::try_from(*card))
            .map(Result::unwrap)
            .collect::<Vec<_>>();

        let gold_cards: Vec<_> = silver_cards.iter().map(|card| card.into_gold()).collect();

        Ok((
            input,
            Line {
                hand: (silver_cards.into(), gold_cards.into()),
                bid,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::{gold, parse::parse, silver, HandType};

    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_parse() {
        let mut parsed = parse(INPUT).into_iter().map(|line| line.hand.0.hand_type);
        assert_eq!(parsed.next(), Some(HandType::OnePair));
        assert_eq!(parsed.next(), Some(HandType::ThreeOfAKind));
        assert_eq!(parsed.next(), Some(HandType::TwoPair));
        assert_eq!(parsed.next(), Some(HandType::TwoPair));
        assert_eq!(parsed.next(), Some(HandType::ThreeOfAKind));
        assert_eq!(parsed.next(), None);
    }

    #[test]
    fn test_silver() {
        let mut parsed = parse(INPUT);
        assert_eq!(silver(&mut parsed), 6440);
    }
    #[test]
    fn test_gold() {
        let mut parsed = parse(INPUT);
        assert_eq!(gold(&mut parsed), 5905);
    }
}
