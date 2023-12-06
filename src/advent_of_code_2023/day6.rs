use crate::aoc_result::AOCResult;

pub fn solve() -> AOCResult {
    (0, 0).into()
}

#[derive(Debug, PartialEq, Eq)]
struct Race {
    time: u8,
    distance: u16,
}

mod parse {
    use nom::IResult;

    use super::Race;

    pub fn parse(input: &str) -> Vec<Race> {
        parse_internal(input).unwrap().1
    }

    fn parse_internal(input: &str) -> IResult<&str, Vec<Race>> {
        let (input, times) = nom::sequence::preceded(
            nom::sequence::pair(
                nom::bytes::complete::tag("Time:"),
                nom::character::complete::space1,
            ),
            nom::multi::separated_list1(
                nom::character::complete::space1,
                nom::character::complete::u8,
            ),
        )(input)?;

        let mut times = times.into_iter();
        let (input, races) = nom::sequence::preceded(
            nom::bytes::complete::tag("\nDistance:"),
            nom::multi::fold_many1(
                nom::sequence::preceded(
                    nom::character::complete::space1,
                    nom::character::complete::u16,
                ),
                Vec::new,
                |mut acc, distance| {
                    let time = times.next().unwrap();
                    acc.push(Race { time, distance });
                    acc
                },
            ),
        )(input)?;

        Ok((input, races))
    }
}

#[cfg(test)]
mod test {
    use super::{parse::parse, Race};

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_parse() {
        let parsed = parse(INPUT);

        assert_eq!(
            parsed,
            [
                Race {
                    time: 7,
                    distance: 9
                },
                Race {
                    time: 15,
                    distance: 40
                },
                Race {
                    time: 30,
                    distance: 200
                },
            ]
        )
    }
}
