use std::ops::RangeInclusive;

use crate::{aoc_result::AOCResult, read_file::read_file};

pub fn solve() -> AOCResult {
    let input = read_file(2023, 5).expect("File input/2023/05.txt to exist");
    let parsed = parse(&input);
    (silver(&parsed), gold(&parsed)).into()
}

fn silver(input: &Input) -> i64 {
    input
        .seeds
        .iter()
        .map(|seed| {
            input
                .rules
                .iter()
                .fold(*seed, |origin, rule| rule.apply(origin))
        })
        .min()
        .unwrap_or(i64::MAX)
}

fn gold(input: &Input) -> i64 {
    let (min, _): (i64, Option<i64>) =
        input
            .seeds
            .iter()
            .fold((i64::MAX, None), |(acc, maybe_previous), cur| {
                if let Some(previous) = maybe_previous {
                    let seed_range_min = (previous..=(previous + cur))
                        .map(|seed| gold_solve_part(&input.rules, seed))
                        .min()
                        .unwrap_or(i64::MAX);

                    (std::cmp::min(acc, seed_range_min), None)
                } else {
                    (acc, Some(*cur))
                }
            });

    min
}

fn gold_solve_part(rules: &[Rule], seed: i64) -> i64 {
    rules.iter().fold(seed, |origin, rule| rule.apply(origin))
}

struct Input {
    seeds: Vec<i64>,
    rules: Vec<Rule>,
}

struct Rule {
    ranges: Vec<Translation>,
}

impl Rule {
    fn apply(&self, origin: i64) -> i64 {
        self.ranges
            .iter()
            .find_map(|translation| translation.apply_translation(origin))
            .unwrap_or(origin)
    }
}

impl From<Vec<Translation>> for Rule {
    fn from(ranges: Vec<Translation>) -> Self {
        Self { ranges }
    }
}

struct Translation {
    source_range: RangeInclusive<i64>,
    source_to_destination_offset: i64,
}

impl Translation {
    fn apply_translation(&self, to_translate: i64) -> Option<i64> {
        if self.source_range.contains(&to_translate) {
            Some(to_translate + self.source_to_destination_offset)
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Input {
    parse::parse_internal(input).unwrap().1
}

mod parse {
    use nom::IResult;

    use super::{Input, Rule, Translation};

    pub fn parse_internal(input: &str) -> IResult<&str, Input> {
        let (input, _) = nom::bytes::complete::tag("seeds: ")(input)?;
        let (input, seeds) = nom::multi::separated_list1(
            nom::character::complete::space1,
            nom::character::complete::i64,
        )(input)?;
        let (input, rules) = nom::multi::fold_many1(
            nom::sequence::preceded(
                nom::sequence::tuple((
                    nom::multi::many1(nom::character::complete::newline),
                    nom::character::complete::alpha1,
                    nom::bytes::complete::tag("-to-"),
                    nom::character::complete::alpha1,
                    nom::bytes::complete::tag(" map:"),
                )),
                nom::multi::fold_many1(
                    nom::sequence::preceded(
                        nom::character::complete::newline,
                        nom::multi::separated_list1(
                            nom::bytes::complete::tag(" "),
                            nom::character::complete::i64,
                        ),
                    ),
                    Vec::new,
                    |mut acc: Vec<Translation>, input: Vec<i64>| {
                        let destination_range_start = input[0];
                        let source_range_start = input[1];
                        let range_length = input[2];

                        acc.push(Translation {
                            source_range: source_range_start..=source_range_start + range_length,
                            source_to_destination_offset: destination_range_start
                                - source_range_start,
                        });

                        acc
                    },
                ),
            ),
            Vec::new,
            |mut acc: Vec<Rule>, item| {
                acc.push(item.into());
                acc
            },
        )(input)?;

        Ok((input, Input { seeds, rules }))
    }
}

#[cfg(test)]
mod test {
    use super::{gold, parse, silver};

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    #[test]
    fn test_silver() {
        let parsed = parse(INPUT);
        assert_eq!(silver(&parsed), 35);
    }

    #[test]
    fn test_gold() {
        let parsed = parse(INPUT);
        assert_eq!(gold(&parsed), 46);
    }
}
