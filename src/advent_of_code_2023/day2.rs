use crate::aoc_result::AOCResult;
use crate::read_file::read_file;

pub fn solve() -> AOCResult {
    let content = read_file(2023, 2).expect("File input/2023/02.txt to exist");
    let parsed = content.lines().map(parse::line).map(Option::unwrap);

    (silver(parsed.clone()), gold(parsed)).into()
}

type LineInfo = (u8, Line);

fn silver(lines: impl Iterator<Item = LineInfo>) -> u32 {
    const ESTIMATED_BAG: Line = Line::rgb(12, 13, 14);

    lines.fold(0, |acc, (game_id, line)| {
        if line.game_is_possible_with(ESTIMATED_BAG) {
            acc + u32::from(game_id)
        } else {
            acc
        }
    })
}

fn gold(lines: impl Iterator<Item = LineInfo>) -> u32 {
    lines.map(|(_, line)| line).map(Line::gold).sum()
}

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Eq)]
struct Line {
    red: u8,
    green: u8,
    blue: u8,
}

impl Line {
    const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self {
            red: r,
            green: g,
            blue: b,
        }
    }

    const fn game_is_possible_with(self, bag: Self) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }

    fn gold(self) -> u32 {
        u32::from(self.red) * u32::from(self.green) * u32::from(self.blue)
    }
}

mod parse {
    use nom::IResult;

    use super::Line;

    pub fn line(input: &str) -> Option<(u8, Line)> {
        line_internal(input).ok().map(|(_, value)| value)
    }

    fn line_internal(input: &str) -> IResult<&str, (u8, Line)> {
        let (input, _) = nom::bytes::complete::tag("Game ")(input)?;
        let (input, game_id) = nom::character::complete::u8(input)?;
        let (input, _) = nom::bytes::complete::tag(": ")(input)?;
        let (input, colors) = nom::multi::separated_list1(
            nom::sequence::tuple((
                nom::character::complete::one_of(";,"),
                nom::bytes::complete::tag(" "),
            )),
            nom::sequence::tuple((
                nom::sequence::terminated(
                    nom::character::complete::u8,
                    nom::bytes::complete::tag(" "),
                ),
                nom::character::complete::alpha1,
            )),
        )(input)?;

        let mut line = Line::default();
        for (num, color) in colors {
            match color {
                "green" => line.green = line.green.max(num),
                "red" => line.red = line.red.max(num),
                "blue" => line.blue = line.blue.max(num),
                _ => panic!(r#"Color should be one of "red"|"green"|"blue", but got {color}"#),
            }
        }

        Ok((input, (game_id, line)))
    }
}

#[cfg(test)]
mod test {
    use super::{gold, parse::line, silver, Line, LineInfo};

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_parse_line() {
        let lines: Vec<_> = test_parse().collect();

        assert_eq!(
            lines,
            vec![
                (1, Line::rgb(4, 2, 6)),
                (2, Line::rgb(1, 3, 4)),
                (3, Line::rgb(20, 13, 6)),
                (4, Line::rgb(14, 3, 15)),
                (5, Line::rgb(6, 3, 2)),
            ]
        )
    }

    #[test]
    fn test_silver() {
        let lines = test_parse();
        assert_eq!(silver(lines), 8);
    }

    #[test]
    fn test_gold() {
        let lines = test_parse();
        assert_eq!(gold(lines), 2286);
    }

    fn test_parse() -> impl Iterator<Item = LineInfo> {
        INPUT.lines().map(line).map(Option::unwrap)
    }
}
