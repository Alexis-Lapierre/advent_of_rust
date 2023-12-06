use crate::{aoc_result::AOCResult, read_file::read_file};

pub fn solve() -> AOCResult {
    let input = read_file(2023, 6).expect("File input/2023/06.txt to exist");
    let (silver_input, gold) = parse::parse(&input);
    (silver(&silver_input), nbr_possibility_beat_record(&gold)).into()
}

fn silver(races: &[Race]) -> usize {
    races.iter().map(nbr_possibility_beat_record).product()
}

fn nbr_possibility_beat_record(race: &Race) -> usize {
    // 0 is never going to win, and it's not 1..=race.time is also not going to win.
    (1..race.time)
        .filter(|button_pressed_for| {
            let speed = u64::from(*button_pressed_for);
            let remaining_time = u64::from(race.time) - speed;

            speed * remaining_time > race.record_distance
        })
        .count()
}

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u32,
    record_distance: u64,
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
                        record_distance: distance,
                    });
                    acc
                },
            ),
        )(input)?;

        let gold_race = silver_races.iter().fold(
            Race {
                time: 0,
                record_distance: 0,
            },
            |mut gold_race, race| {
                gold_race.time = gold_race.time * 100 + race.time;

                let multiplier = match race.record_distance {
                    0..=9 => 10,
                    10..=99 => 100,
                    100..=999 => 1000,
                    1000..=9_999 => 10_000,
                    _ => panic!("Unexpected time over 10_000 (got {})", race.record_distance),
                };

                gold_race.record_distance =
                    gold_race.record_distance * multiplier + race.record_distance;

                gold_race
            },
        );

        Ok((input, (silver_races, gold_race)))
    }
}

#[cfg(test)]
mod test {
    use super::{parse::parse, silver, Race};

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
                    record_distance: 9
                },
                Race {
                    time: 15,
                    record_distance: 40
                },
                Race {
                    time: 30,
                    record_distance: 200
                },
            ]
        );
        assert_eq!(
            gold,
            Race {
                time: 71530,
                record_distance: 940_200
            },
        );
    }

    #[test]
    fn test_silver() {
        let (parsed, _) = parse(INPUT);
        assert_eq!(silver(&parsed), 288);
    }
}
