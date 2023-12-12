use nom::{
    bytes::complete::tag,
    character::complete::{newline, one_of},
    multi, sequence, IResult,
};

use crate::{aoc_result::AOCResult, read_file::read_file};

pub fn solve() -> AOCResult {
    let input = read_file(2023, 12).expect("Read file should be correct for 2023 12");
    let parsed = parse(&input);
    (silver(&parsed), 0).into()
}

fn silver(lines: &[Line]) -> usize {
    lines
        .iter()
        .map(|line| {
            let count = line
                .silver_possibilities()
                .map(|possibility| line.is_possible_with(possibility))
                .filter(|e| *e)
                .count();

            if count == 0 {
                1
            } else {
                count
            }
        })
        .sum()
}

#[derive(Debug)]
struct Line {
    history: Vec<Option<State>>,
    group_damaged: Vec<u8>,
}

impl Line {
    fn is_possible_with(&self, mut possibility: SilverPossibility) -> bool {
        let group_damaged = self
            .history
            .iter()
            .map(|state| match state {
                None => possibility.next().unwrap(),
                Some(state) => *state,
            })
            .fold(
                (Vec::new(), State::Working),
                |(mut acc, previous_state): (Vec<u8>, State), cur| {
                    match (cur, previous_state) {
                        (State::Working, _) => (),
                        (State::Broken, State::Broken) => {
                            *acc.last_mut().unwrap() += 1;
                        }
                        (State::Broken, State::Working) => {
                            acc.push(1);
                        }
                    };

                    (acc, cur)
                },
            )
            .0;

        group_damaged == self.group_damaged
    }

    fn silver_possibilities(&self) -> impl Iterator<Item = SilverPossibility> {
        let nbr_damaged: u8 = self.group_damaged.iter().sum();
        let already_damaged_in_history = u8::try_from(
            self.history
                .iter()
                .filter(|state| **state == Some(State::Broken))
                .count(),
        )
        .unwrap();

        let missing_damaged = u32::from(nbr_damaged - already_damaged_in_history);
        let questionmark_in_history =
            u8::try_from(self.history.iter().filter(|state| **state == None).count()).unwrap();

        (1..2u32.pow(u32::from(questionmark_in_history)))
            .into_iter()
            .filter(move |index| index.count_ones() == missing_damaged)
            .map(SilverPossibility::from)
    }
}

impl From<(Vec<Option<State>>, Vec<u8>)> for Line {
    fn from((history, group_damaged): (Vec<Option<State>>, Vec<u8>)) -> Self {
        Line {
            history,
            group_damaged,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Working,
    Broken,
}

struct SilverPossibility {
    is_broken_list: u32,
}

impl From<u32> for SilverPossibility {
    fn from(value: u32) -> Self {
        Self {
            is_broken_list: value,
        }
    }
}

impl Iterator for SilverPossibility {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        let is_broken = (self.is_broken_list & 1) == 1;
        self.is_broken_list = self.is_broken_list >> 1;

        Some(if is_broken {
            State::Broken
        } else {
            State::Working
        })
    }
}

fn parse(input: &str) -> Vec<Line> {
    parse_internal(input).unwrap().1
}

fn parse_internal(input: &str) -> IResult<&str, Vec<Line>> {
    let (input, result) = multi::separated_list1(
        newline,
        sequence::pair(
            multi::fold_many1(
                one_of("?.#"),
                Vec::new,
                |mut acc: Vec<Option<State>>, elem: char| {
                    acc.push(match elem {
                        '?' => None,
                        '.' => Some(State::Working),
                        '#' => Some(State::Broken),
                        _ => panic!("Unexpected elem, should not happen"),
                    });

                    acc
                },
            ),
            sequence::preceded(
                tag(" "),
                multi::separated_list1(tag(","), nom::character::complete::u8),
            ),
        ),
    )(input)?;

    Ok((input, result.into_iter().map(Line::from).collect()))
}

#[cfg(test)]
mod test {
    use super::{parse, silver};

    const INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    #[test]
    fn test_silver() {
        let parsed = parse(INPUT);
        assert_eq!(silver(&parsed), 21);
    }

    #[test]
    fn parse_line() {
        // No possibility because no question mark to do...
        assert_eq!(silver(&parse("??.#???.#? 1,1")), 1);
    }
}
