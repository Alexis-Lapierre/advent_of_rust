use crate::{aoc_result::AOCResult, read_file::read_file};

pub fn solve() -> AOCResult {
    let input = read_file(2023, 6).expect("File input/2023/06.txt to exist");
    let (silver_input, gold) = parse::parse(&input);
    (silver(&silver_input), nbr_possibility_beat_record(&gold)).into()
}

fn silver(races: &[Race]) -> u64 {
    races.iter().map(nbr_possibility_beat_record).product()
}

#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn nbr_possibility_beat_record(race: &Race) -> u64 {
    let time = f64::from(race.time);

    // don't really know how to avoid precision loss, but should not impact regular aoc input
    #[allow(clippy::cast_precision_loss)]
    let distance_record = (race.distance_record + 1) as f64;

    let root = 4_f64.mul_add(-distance_record, time.powi(2)).sqrt();
    let lower = ((time - root) / 2.).ceil();
    let upper = ((time + root) / 2.).floor();

    (upper as u64) - (lower as u64) + 1
}

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u32,
    distance_record: u64,
}

mod parse {
    use nom::IResult;

    use super::Race;

    pub fn parse(input: &str) -> (Vec<Race>, Race) {
        parse_internal(input).unwrap().1
    }

    fn parse_internal(input: &str) -> IResult<&str, (Vec<Race>, Race)> {
        let (input, times) = nom::sequence::preceded(
            nom::sequence::pair(
                nom::bytes::complete::tag("Time:"),
                nom::character::complete::space1,
            ),
            nom::multi::separated_list1(
                nom::character::complete::space1,
                nom::character::complete::u32,
            ),
        )(input)?;

        let mut times = times.into_iter();
        let (input, silver_races) = nom::sequence::preceded(
            nom::bytes::complete::tag("\nDistance:"),
            nom::multi::fold_many1(
                nom::sequence::preceded(
                    nom::character::complete::space1,
                    nom::character::complete::u64,
                ),
                Vec::new,
                |mut acc, distance| {
                    let time = times.next().unwrap();
                    acc.push(Race {
                        time,
                        distance_record: distance,
                    });
                    acc
                },
            ),
        )(input)?;

        let gold_race = silver_races.iter().fold(
            Race {
                time: 0,
                distance_record: 0,
            },
            |mut gold_race, race| {
                gold_race.time = gold_race.time * 100 + race.time;

                let multiplier = match race.distance_record {
                    0..=9 => 10,
                    10..=99 => 100,
                    100..=999 => 1000,
                    1000..=9_999 => 10_000,
                    _ => panic!("Unexpected time over 10_000 (got {})", race.distance_record),
                };

                gold_race.distance_record =
                    gold_race.distance_record * multiplier + race.distance_record;

                gold_race
            },
        );

        Ok((input, (silver_races, gold_race)))
    }
}

#[cfg(test)]
mod test {
    use super::{nbr_possibility_beat_record, parse::parse, silver, Race};

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_parse() {
        let (silver, gold) = parse(INPUT);

        assert_eq!(
            silver,
            [
                Race {
                    time: 7,
                    distance_record: 9
                },
                Race {
                    time: 15,
                    distance_record: 40
                },
                Race {
                    time: 30,
                    distance_record: 200
                },
            ]
        );
        assert_eq!(
            gold,
            Race {
                time: 71530,
                distance_record: 940_200
            },
        );
    }

    #[test]
    fn test_silver() {
        let (parsed, gold_parsed) = parse(INPUT);
        assert_eq!(nbr_possibility_beat_record(&gold_parsed), 71503);
        let mut silver_iter = parsed.iter().map(nbr_possibility_beat_record);
        assert_eq!(silver_iter.next(), Some(4));
        assert_eq!(silver_iter.next(), Some(8));
        assert_eq!(silver_iter.next(), Some(9));
        assert_eq!(silver_iter.next(), None);
        assert_eq!(silver(&parsed), 288);
    }
}
