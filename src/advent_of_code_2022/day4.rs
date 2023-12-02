use crate::{aoc_result::AOCResult, read_file::read_file};

type Range = (u32, u32);
type Line = (Range, Range);

pub fn solve() -> AOCResult {
    let content = read_file(2022, 4).expect("File input/2022/04.txt to exist");
    let parsed = parse::lines(&content).expect("expected parsing to be successful");

    (silver(&parsed), gold(&parsed)).into()
}

fn silver(lines: &[Line]) -> usize {
    lines.iter().filter(|line| silver_line(line)).count()
}

fn silver_line(line: &Line) -> bool {
    let ((left_start, left_end), (right_start, right_end)) = line;
    (left_start <= right_start && left_end >= right_end)
        || (left_start >= right_start && left_end <= right_end)
}

fn gold(lines: &[Line]) -> usize {
    lines.iter().filter(|line| gold_line(line)).count()
}

fn gold_line(line: &Line) -> bool {
    let ((left_start, left_end), (right_start, right_end)) = line;
    (left_start >= right_start && left_start <= right_end)
        || (left_end >= right_start && left_end <= right_end)
        || (right_start >= left_start && right_start <= left_end)
        || (right_end >= left_start && right_end <= left_end)
}

mod parse {
    use nom::{
        self,
        bytes::complete::tag,
        character::complete::{newline, u32 as nom_u32},
        multi::separated_list1,
        sequence::separated_pair,
        IResult,
    };

    use super::{Line, Range};

    fn elf_section(input: &str) -> IResult<&str, Range> {
        separated_pair(nom_u32, tag("-"), nom_u32)(input)
    }

    fn line(input: &str) -> IResult<&str, Line> {
        separated_pair(elf_section, tag(","), elf_section)(input)
    }

    pub fn lines(input: &str) -> Result<Vec<Line>, nom::Err<nom::error::Error<&str>>> {
        separated_list1(newline, line)(input).map(|(_, lines)| lines)
    }
}

#[cfg(test)]
mod tests {
    use super::{gold, parse, silver, silver_line, Line};

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_parse() {
        let lines = parse::lines(INPUT).unwrap();
        assert_eq!(
            lines,
            vec![
                ((2, 4), (6, 8)),
                ((2, 3), (4, 5)),
                ((5, 7), (7, 9)),
                ((2, 8), (3, 7)),
                ((6, 6), (4, 6)),
                ((2, 6), (4, 8)),
            ]
        );
    }

    #[test]
    fn test_silver_line() {
        const LINE: Line = ((6, 6), (3, 6));
        assert!(silver_line(&LINE));
    }

    #[test]
    fn test_silver() {
        assert_eq!(silver(&parse::lines(INPUT).unwrap()), 2);
    }

    #[test]
    fn test_gold() {
        assert_eq!(gold(&parse::lines(INPUT).unwrap()), 4);
    }
}
