use crate::read_file::read_file;

pub fn solve() -> (u32, u32) {
    let content = read_file(2023, 1).expect("File input/2023/01.txt to exist");

    let silver = parse::lines(&content, &parse::silver_parser);
    let gold = parse::lines(&content, &parse::gold_parser);

    (sum(silver), sum(gold))
}

fn sum(lines: impl Iterator<Item = LineInfo>) -> u32 {
    lines.map(|l| l.to_result()).sum()
}

#[derive(Debug, PartialEq, Eq)]
struct LineInfo {
    start: u32,
    end: u32,
}

impl LineInfo {
    const fn to_result(&self) -> u32 {
        self.start * 10 + self.end
    }

    const fn from(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    const fn progressive_parse(&self, new_end: u32) -> Self {
        Self::from(self.start, new_end)
    }
}

mod parse {
    use nom::{
        bytes::complete::{tag, take},
        combinator::map_res,
        IResult,
    };

    use super::LineInfo;

    pub fn lines<'a, F>(input: &'a str, parser: &'a F) -> impl Iterator<Item = LineInfo> + 'a
    where
        F: Fn(&str) -> IResult<&str, u32> + 'a,
    {
        let line_parser = line(parser);
        input.lines().map(line_parser)
    }

    fn line<'a, F>(parser: &'a F) -> impl Fn(&str) -> LineInfo + 'a
    where
        F: Fn(&str) -> IResult<&str, u32> + 'a,
    {
        |input: &str| {
            let mut previous_input = input;
            let mut find_first = || loop {
                if let Ok((_, found)) = parser(previous_input) {
                    return LineInfo::from(found, found);
                };
                // line should always contain at least one number
                // hence the unwrap
                previous_input = take_one(previous_input).unwrap();
            };

            let mut result = find_first();
            while let Some(next_value) = take_one(previous_input) {
                if let Ok((_, value)) = parser(previous_input) {
                    result = result.progressive_parse(value);
                };

                previous_input = next_value;
            }

            result
        }
    }

    pub fn gold_parser(input: &str) -> IResult<&str, u32> {
        let lambda = |n: u32| move |_| -> Result<u32, &str> { Ok(n) };

        nom::branch::alt((
            silver_parser,
            map_res(tag("one"), lambda(1)),
            map_res(tag("two"), lambda(2)),
            map_res(tag("three"), lambda(3)),
            map_res(tag("four"), lambda(4)),
            map_res(tag("five"), lambda(5)),
            map_res(tag("six"), lambda(6)),
            map_res(tag("seven"), lambda(7)),
            map_res(tag("eight"), lambda(8)),
            map_res(tag("nine"), lambda(9)),
        ))(input)
    }

    pub fn silver_parser(input: &str) -> IResult<&str, u32> {
        map_res(
            nom::character::complete::one_of("123456789"),
            |s| -> Result<u32, &str> { s.to_digit(10).ok_or(input) },
        )(input)
    }

    fn take_one(input: &str) -> Option<&str> {
        take_one_lambda(input).map(|(next, _)| next).ok()
    }

    fn take_one_lambda(input: &str) -> IResult<&str, &str> {
        take(1_usize)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, sum, LineInfo};

    const SILVER_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    const GOLD_INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    const SILVER_INPUT_EXPECTED: [LineInfo; 4] = [
        LineInfo::from(1, 2),
        LineInfo::from(3, 8),
        LineInfo::from(1, 5),
        LineInfo::from(7, 7),
    ];

    const GOLD_INPUT_EXPECTED: [LineInfo; 7] = [
        LineInfo::from(2, 9),
        LineInfo::from(8, 3),
        LineInfo::from(1, 3),
        LineInfo::from(2, 4),
        LineInfo::from(4, 2),
        LineInfo::from(1, 4),
        LineInfo::from(7, 6),
    ];

    #[test]
    fn test_parse() {
        let lines: Vec<LineInfo> = parse::lines(SILVER_INPUT, &parse::silver_parser).collect();
        assert_eq!(lines, SILVER_INPUT_EXPECTED);
    }

    #[test]
    fn test_gold_parse() {
        let lines = parse::lines(SILVER_INPUT, &parse::gold_parser).collect::<Vec<LineInfo>>();
        assert_eq!(lines, SILVER_INPUT_EXPECTED);

        let lines: Vec<LineInfo> = parse::lines(GOLD_INPUT, &parse::gold_parser).collect();

        assert_eq!(lines, GOLD_INPUT_EXPECTED);
    }

    #[test]
    fn test_gold() {
        let lines = parse::lines(SILVER_INPUT, &parse::gold_parser);
        assert_eq!(sum(lines), 142);
        let lines = parse::lines(GOLD_INPUT, &parse::gold_parser);
        assert_eq!(sum(lines), 281);
    }
}
