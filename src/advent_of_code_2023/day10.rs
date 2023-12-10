use std::{collections::HashMap, iter, ops::RangeInclusive};

use crate::{aoc_result::AOCResult, read_file::read_file};

pub fn solve() -> AOCResult {
    let input = read_file(2023, 10).expect("File input/2023/10.txt");
    let parsed = parse::parse(&input);
    (silver(parsed), 0).into()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Copy, Clone)]
enum Pipe {
    DownLeft,
    DownRight,
    LeftRight,
    UpDown,
    UpLeft,
    UpRight,
}

impl TryFrom<u8> for Pipe {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'7' => Ok(Self::DownLeft),
            b'.' => Err("No pipe here"),
            b'F' => Ok(Self::DownRight),
            b'J' => Ok(Self::UpLeft),
            b'L' => Ok(Self::UpRight),
            b'-' => Ok(Self::LeftRight),
            b'|' => Ok(Self::UpDown),
            _ => Err("Invalid character encountered"),
        }
    }
}

impl Pipe {
    const fn offsets(self) -> ((i8, i8), (i8, i8)) {
        const DOWN: (i8, i8) = (0, 1);
        const UP: (i8, i8) = (0, -1);
        const LEFT: (i8, i8) = (-1, 0);
        const RIGHT: (i8, i8) = (1, 0);
        match self {
            Self::DownLeft => (DOWN, LEFT),
            Self::DownRight => (DOWN, RIGHT),
            Self::LeftRight => (LEFT, RIGHT),
            Self::UpDown => (UP, DOWN),
            Self::UpLeft => (UP, LEFT),
            Self::UpRight => (UP, RIGHT),
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd)]
struct Point {
    x: u8,
    y: u8,
}

impl Point {
    const fn new((x, y): (u8, u8)) -> Self {
        Self { x, y }
    }

    fn list_neighbors(&self) -> impl Iterator<Item = Self> + '_ {
        let (x_range, y_range) = self.get_ranges();

        x_range
            .into_iter()
            .map(move |x| iter::repeat(x).zip(y_range.clone()).map(Self::new))
            .flatten()
            .filter(|point| *self != *point)
    }

    fn list_neighbors_pointing_to_self<'a>(
        &'a self,
        map: &'a HashMap<Point, Pipe>,
    ) -> impl Iterator<Item = Point> + 'a {
        self.list_neighbors()
            .map(|neighbors| map.get_key_value(&neighbors))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .filter(|(point, pipe)| {
                let offsets = pipe.offsets();

                point.from_offset(offsets.0) == Some(*self)
                    || point.from_offset(offsets.1) == Some(*self)
            })
            .map(|(point, _)| *point)
    }

    const fn get_ranges(self) -> (RangeInclusive<u8>, RangeInclusive<u8>) {
        let x_range = self.x.saturating_sub(1)..=self.x.saturating_add(1);
        let y_range = self.y.saturating_sub(1)..=self.y.saturating_add(1);

        (x_range, y_range)
    }

    fn from_offset(self, offset: (i8, i8)) -> Option<Self> {
        let (x, y) = offset;

        match (checked_add(self.x, x), checked_add(self.y, y)) {
            (Some(x), Some(y)) => Some(Self::new((x, y))),
            _ => None,
        }
    }
}

fn pipe_at_point_to(pipe: Pipe, current_position: Point) -> (Point, Point) {
    let offsets = pipe.offsets();

    (
        current_position.from_offset(offsets.0).unwrap(),
        current_position.from_offset(offsets.1).unwrap(),
    )
}

fn pipe_at_point_next(pipe: Pipe, current_position: Point, previous_position: Point) -> Point {
    let (left, right) = pipe_at_point_to(pipe, current_position);
    if left == previous_position {
        right
    } else {
        left
    }
}

fn checked_add(n: u8, i: i8) -> Option<u8> {
    let signed = u8::try_from(i.abs()).unwrap();

    if i.is_negative() {
        n.checked_sub(signed)
    } else {
        n.checked_add(signed)
    }
}

mod parse {
    use crate::advent_of_code_2023::day10::Point;

    use super::Pipe;
    use std::collections::HashMap;

    pub fn parse(input: &str) -> (HashMap<Point, Pipe>, Point) {
        let mut map = HashMap::new();
        let mut start = None;
        for (l, y) in input.lines().zip(0u8..) {
            let new_start = line(l, y, &mut map);
            start = start.or(new_start);
        }

        (map, start.unwrap())
    }

    fn line(input: &str, y: u8, map: &mut HashMap<Point, Pipe>) -> Option<Point> {
        let mut maybe_start = None;
        for (char, x) in input.bytes().zip(0u8..) {
            let point = Point::new((x, y));
            if let Ok(pipe) = Pipe::try_from(char) {
                assert_eq!(map.insert(point, pipe), None);
            } else if char == b'S' {
                maybe_start = Some(point);
            }
        }

        maybe_start
    }
}

fn silver((map, start): (HashMap<Point, Pipe>, Point)) -> u32 {
    let mut iter = start.list_neighbors_pointing_to_self(&map);
    let mut left = iter.next().unwrap();
    let mut previous_left = start;
    let mut right = iter.next().unwrap();
    let mut previous_right = start;

    for distance in 2u32.. {
        let new_left = pipe_at_point_next(*map.get(&left).unwrap(), left, previous_left);
        let new_right = pipe_at_point_next(*map.get(&right).unwrap(), right, previous_right);

        previous_left = left;
        left = new_left;
        previous_right = right;
        right = new_right;

        if left == right {
            return distance;
        }
    }

    panic!()
}

#[cfg(test)]
mod test {
    use super::{parse::parse, silver, Point};

    const SIMPLE_LOOP: &str = ".....
.S-7.
.|.|.
.L-J.
.....";
    const COMPLEX_LOOP: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    #[test]
    fn test_find_start() {
        let parsed = parse(SIMPLE_LOOP);
        assert_eq!(parsed.1, Point::new((1, 1)));
    }

    #[test]
    fn test_silver() {
        let parsed = parse(SIMPLE_LOOP);
        assert_eq!(silver(parsed), 4);
        let parsed = parse(COMPLEX_LOOP);
        assert_eq!(silver(parsed), 8);
    }
}
