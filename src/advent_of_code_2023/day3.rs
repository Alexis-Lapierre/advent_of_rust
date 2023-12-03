use crate::aoc_result::AOCResult;
use crate::read_file::read_file;

pub fn solve() -> AOCResult {
    let content = read_file(2023, 3).expect("File input/2023/03.txt to exist");
    let parsed = parse::parse(&content).unwrap();

    (silver(&parsed), gold(&parsed)).into()
}

fn silver(input: &Input) -> u32 {
    todo!()
}

fn gold(input: &Input) -> u32 {
    0
}

#[derive(Debug, PartialEq, Default)]
struct Input {
    parts: Vec<PartNumber>,
    symbol: Vec<Point>,
}

impl Input {
    fn add_symbol(&mut self, position: Point) {
        self.symbol.push(position);
    }

    fn add_part(&mut self, part_number_to_parse: &str, start_position: Point) {
        let length = u8::try_from(part_number_to_parse.len()).unwrap();
        let value: u16 = part_number_to_parse.parse().unwrap();
        self.parts.push(PartNumber {
            start_position,
            length,
            value,
        });
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    fn new_line(&mut self) {
        self.y += 1;
        self.x = 0;
    }

    fn move_right(&mut self, count: u8) {
        self.x += count;
    }
}

impl From<(u8, u8)> for Point {
    fn from(point: (u8, u8)) -> Self {
        let (x, y) = point;
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PartNumber {
    start_position: Point,
    length: u8,
    value: u16,
}

impl From<(u8, u8, u8, u16)> for PartNumber {
    fn from(input: (u8, u8, u8, u16)) -> Self {
        let (x, y, length, value) = input;
        Self {
            start_position: (x, y).into(),
            length,
            value,
        }
    }
}

mod parse {
    use nom::IResult;

    use super::{Input, Point};

    pub fn parse(lines: &str) -> Option<Input> {
        parse_internal(lines).map(|(_, input)| input).ok()
    }

    fn parse_internal(lines: &str) -> IResult<&str, Input> {
        let (lines, (input, _)) = nom::multi::fold_many1(
            nom::branch::alt((
                nom::bytes::complete::tag("#"),
                nom::bytes::complete::tag("*"),
                nom::bytes::complete::tag("+"),
                nom::bytes::complete::tag("."),
                nom::bytes::complete::tag("$"),
                nom::bytes::complete::tag("\n"),
                nom::character::complete::digit1,
            )),
            || (Input::default(), Point::default()),
            |(mut input, mut point), captured: &str| {
                if captured == "\n" {
                    point.new_line();
                } else {
                    match captured {
                        "*" | "#" | "+" | "$" => input.add_symbol(point),
                        "." => (),
                        number => input.add_part(number, point),
                    };
                    point.move_right(u8::try_from(captured.len()).unwrap())
                }
                (input, point)
            },
        )(lines)?;

        Ok((dbg!(lines), input))
    }
}

mod test {
    use super::{parse, Input, PartNumber, Point};

    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_parse_input() {
        let parsed = parse::parse(INPUT).unwrap();
        let mut parts = parsed.parts.iter();
        assert_eq!(parts.next(), Some((0, 0, 3, 467).into()).as_ref());
        assert_eq!(parts.next(), Some((5, 0, 3, 114).into()).as_ref());
        assert_eq!(parts.next(), Some((2, 2, 2, 35).into()).as_ref());
        assert_eq!(parts.next(), Some((6, 2, 3, 633).into()).as_ref());
        assert_eq!(parts.next(), Some((0, 4, 3, 617).into()).as_ref());
        assert_eq!(parts.next(), Some((7, 5, 2, 58).into()).as_ref());
        assert_eq!(parts.next(), Some((2, 6, 3, 592).into()).as_ref());
        assert_eq!(parts.next(), Some((6, 7, 3, 755).into()).as_ref());
        assert_eq!(parts.next(), Some((1, 9, 3, 664).into()).as_ref());
        assert_eq!(parts.next(), Some((5, 9, 3, 598).into()).as_ref());

        assert_eq!(
            parsed.symbol,
            vec![
                Point { x: 3, y: 1 },
                Point { x: 6, y: 3 },
                Point { x: 3, y: 4 },
                Point { x: 5, y: 5 },
                Point { x: 3, y: 8 },
                Point { x: 5, y: 8 }
            ],
        );
    }
}
