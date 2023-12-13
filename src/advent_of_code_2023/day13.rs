use std::collections::HashMap;

use nom::IResult;

use crate::{aoc_result::AOCResult, read_file::read_file};

pub fn solve() -> AOCResult {
    let input = read_file(2023, 13).expect("Read file should be correct for 2023 13");
    let parsed = parse(&input);
    (0, 0).into()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Rock {}

type MirrorMaze = HashMap<(u8, u8), Option<Rock>>;

fn parse(input: &str) -> Vec<MirrorMaze> {
    parse_internal(input).unwrap().1
}

fn parse_internal(input: &str) -> IResult<&str, Vec<MirrorMaze>> {
    let (input, mazes) = nom::multi::separated_list1(
        nom::character::complete::newline,
        nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::character::complete::one_of("#."),
        ),
    )(input)?;

    let mazes = mazes
        .into_iter()
        .zip(0u8..)
        .map(|(maze, x)| {
            maze.into_iter()
                .zip(0u8..)
                .fold(HashMap::new(), |mut acc, (rock, y)| {
                    let maybe_rock = match rock {
                        '#' => Some(Rock {}),
                        '.' => None,
                        _ => panic!("should not happen. {rock} was something else than '#' or '.'"),
                    };

                    assert_eq!(acc.insert((x, y), maybe_rock), None);

                    acc
                })
        })
        .collect();

    Ok((input, mazes))
}
