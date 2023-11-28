use crate::read_file::read_file;

pub fn solve() -> (u32, u32) {
    let content = read_file(2022, 4).expect("File input/2022/04.txt to exist");
    println!("Content is: {:?}", content);
    todo!()
}

type Range = (u32, u32);
type Line = (Range, Range);

mod parse {
    use super::*;
    use nom::{
        self,
        bytes::complete::tag,
        character::complete::{newline, u32 as nom_u32},
        combinator::all_consuming,
        multi::separated_list1,
        sequence::separated_pair,
        IResult,
    };

    fn elf_section(input: &str) -> IResult<&str, Range> {
        separated_pair(nom_u32, tag("-"), nom_u32)(input)
    }

    fn line(input: &str) -> IResult<&str, Line> {
        separated_pair(elf_section, tag(","), elf_section)(input)
    }

    pub fn lines(input: &str) -> Option<Vec<Line>> {
        all_consuming(separated_list1(newline, line))(input)
            .map(|(_str, lines)| lines)
            .ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "2-4,6-8
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
}
