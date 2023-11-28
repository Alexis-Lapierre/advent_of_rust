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
        IResult,
    };

    fn elf_section(input: &str) -> IResult<&str, Range> {
        let (input, start) = nom_u32(input)?;
        let (input, _) = tag("-")(input)?;
        let (input, end) = nom_u32(input)?;

        Ok((input, (start, end)))
    }

    fn line(input: &str) -> IResult<&str, Line> {
        let (input, first) = elf_section(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, second) = elf_section(input)?;

        Ok((input, (first, second)))
    }

    pub fn lines(input: &str) -> Option<Vec<Line>> {
        if let Ok((_, result)) =
            nom::combinator::all_consuming(nom::multi::separated_list1(newline, line))(input)
        {
            Some(result)
        } else {
            None
        }
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
