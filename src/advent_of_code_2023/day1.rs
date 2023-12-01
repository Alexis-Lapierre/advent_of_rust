use crate::read_file::read_file;

pub fn solve() -> (u32, u32) {
    let content = read_file(2023, 1).expect("File input/2023/01.txt to exist");
    let parsed = parse::lines(&content);

    (silver(parsed), 0)
}

fn silver(elems: impl Iterator<Item = u32>) -> u32 {
    elems.sum()
}

mod parse {
    use nom::character::complete::alpha0;

    fn line(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
        let first = first_number(input)?;
        let rev_input = input.chars().rev().collect::<String>();
        let last = first_number(rev_input.as_str()).unwrap();

        Ok(first * 10 + last)
    }

    fn first_number(input: &str) -> Result<u32, nom::Err<nom::error::Error<&str>>> {
        let (input, _) = alpha0(input)?;
        let searched = input.chars().next().unwrap().to_digit(10).unwrap();
        Ok(searched)
    }

    pub fn lines<'a>(input: &'a str) -> impl Iterator<Item = u32> + 'a {
        input.lines().map(line).map(|line| line.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_parse() {
        let lines = parse::lines(INPUT).collect::<Vec<u32>>();
        assert_eq!(lines, vec![12, 38, 15, 77,]);
    }
}
