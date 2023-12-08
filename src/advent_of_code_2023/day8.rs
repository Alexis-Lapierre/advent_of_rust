use std::collections::HashMap;

use crate::{aoc_result::AOCResult, read_file::read_file};

pub fn solve() -> AOCResult {
    let input = read_file(2023, 8).expect("File input/2023/08.txt");
    let program = parse::parse(&input);
    (silver(&program), gold(&program)).into()
}

// ASCII for 'A' 3 times
const AAA: u32 = 0x414141;
const ZZZ: u32 = 0x5A5A5A;

fn silver(program: &Program) -> u32 {
    let instructions = program.instructions.iter().cycle().zip(1u32..);

    let mut current_instruction = AAA;
    for (instruction, index) in instructions {
        let lr = program.map.get(&current_instruction).unwrap();
        current_instruction = *instruction.choose(lr);
        if current_instruction == ZZZ {
            return index;
        }
    }

    panic!("This should not happen !");
}

fn gold(program: &Program) -> u64 {
    find_factors(program)
        .into_iter()
        .map(u64::from)
        .fold(1, num::integer::lcm)
}

fn find_factors(program: &Program) -> Vec<u32> {
    let instructions = program.instructions.iter().copied().cycle().zip(1u32..);

    let mut current_instruction: Vec<u32> = program
        .map
        .keys()
        .copied()
        .filter(|key| end_with(*key, b'A'))
        .collect();

    let capacity = current_instruction.capacity();
    let mut result_cycles: Vec<u32> = Vec::with_capacity(capacity);

    for (instruction, index) in instructions {
        let new_ins = current_instruction
            .into_iter()
            .map(|cur| *instruction.choose(program.map.get(&cur).unwrap()));
        current_instruction = Vec::with_capacity(capacity);
        for current in new_ins {
            if end_with(current, b'Z') {
                result_cycles.push(index);
            } else {
                current_instruction.push(current);
            }
        }

        if current_instruction.is_empty() {
            return result_cycles;
        }
    }

    panic!("Should not happen");
}

const fn end_with(number: u32, test: u8) -> bool {
    number & 0xFF == test as u32
}

#[derive(Debug)]
struct Program {
    instructions: Vec<Instruction>,
    map: HashMap<u32, (u32, u32)>,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

impl Instruction {
    const fn choose<T>(self, tuple: &(T, T)) -> &T {
        match self {
            Self::Left => &tuple.0,
            Self::Right => &tuple.1,
        }
    }
}

impl TryFrom<char> for Instruction {
    type Error = String;

    fn try_from(input: char) -> Result<Self, Self::Error> {
        match input {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(format!("{input} is not either 'L' or 'R'")),
        }
    }
}

mod parse {
    use std::collections::HashMap;

    use nom::{character::complete::alphanumeric1, IResult};

    use super::{Instruction, Program};

    pub fn parse(input: &str) -> Program {
        parse_internal(input).unwrap().1
    }

    fn parse_internal(input: &str) -> IResult<&str, Program> {
        let (input, instructions) = nom::multi::fold_many1(
            nom::character::complete::one_of("LR"),
            Vec::new,
            |mut acc: Vec<Instruction>, elem: char| {
                let instruction: Instruction = elem.try_into().unwrap();
                acc.push(instruction);
                acc
            },
        )(input)?;

        let (input, map) = nom::sequence::preceded(
            nom::multi::many1(nom::character::complete::newline),
            nom::multi::fold_many1(
                nom::sequence::tuple((
                    nom::sequence::terminated(alphanumeric1, nom::bytes::complete::tag(" = (")),
                    nom::sequence::terminated(
                        nom::sequence::separated_pair(
                            alphanumeric1,
                            nom::bytes::complete::tag(", "),
                            alphanumeric1,
                        ),
                        nom::bytes::complete::tag(")\n"),
                    ),
                )),
                HashMap::new,
                |mut map: HashMap<u32, (u32, u32)>, elem| {
                    let key = str_to_u32(elem.0);
                    let tuple = (str_to_u32(elem.1 .0), str_to_u32(elem.1 .1));
                    assert!(map.insert(key, tuple).is_none());
                    map
                },
            ),
        )(input)?;

        Ok((input, Program { instructions, map }))
    }

    pub fn str_to_u32(input: &str) -> u32 {
        input
            .bytes()
            .fold(0, |acc, byte| (acc << 8) + u32::from(byte))
    }
}

#[cfg(test)]
mod test {
    use super::{gold, parse, silver};

    const SILVER_INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    const GOLD_INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[test]
    fn test_silver() {
        let parsed = parse::parse(SILVER_INPUT);
        assert_eq!(silver(&parsed), 6);
    }

    #[test]
    fn test_gold() {
        let parsed = parse::parse(GOLD_INPUT);
        assert_eq!(gold(&parsed), 6);
    }
}
