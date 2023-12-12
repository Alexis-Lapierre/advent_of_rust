use crate::{aoc_result::AOCResult, read_file::read_file};

pub fn solve() -> AOCResult {
    let input = read_file(2023, 11).expect("File input/2023/11.txt");
    let parsed = parse::parse(&input);

    (silver(parsed.clone()), gold(parsed)).into()
}

fn silver(sky: MilkyWay) -> usize {
    let points = get_galaxies_with_empty_offset(sky, 1);
    find_manhatan_distance_for_each(&points)
}

fn gold(sky: MilkyWay) -> usize {
    let points = dbg!(get_galaxies_with_empty_offset(sky, 999999));

    find_manhatan_distance_for_each(&points)
}

#[derive(Debug, Copy, Clone)]
struct Galaxy {}

type MilkyWay = Vec<Vec<Option<Galaxy>>>;

mod parse {
    use nom::IResult;

    use super::Galaxy;

    pub fn parse(input: &str) -> Vec<Vec<Option<Galaxy>>> {
        internal(input).unwrap().1
    }

    fn internal(input: &str) -> IResult<&str, Vec<Vec<Option<Galaxy>>>> {
        let (input, elem) = nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::multi::fold_many1(
                nom::character::complete::one_of(".#"),
                Vec::new,
                |mut acc: Vec<Option<Galaxy>>, elem: char| {
                    match elem {
                        '.' => acc.push(None),
                        '#' => acc.push(Some(Galaxy {})),
                        _ => panic!("unexpected element {elem}, expected either '.' or '#'"),
                    }

                    acc
                },
            ),
        )(input)?;

        Ok((input, elem))
    }
}

fn list_empty_lines(sky: MilkyWay) -> (Vec<usize>, Vec<usize>) {
    let x_result = list_x_empty_line(&sky);
    let sky = rotate(sky);
    let y_result = list_x_empty_line(&sky);

    (x_result, y_result)
}

fn list_x_empty_line(sky: &[Vec<Option<Galaxy>>]) -> Vec<usize> {
    sky.iter()
        .enumerate()
        .filter_map(|(x, line)| {
            if line.iter().all(Option::is_none) {
                Some(x)
            } else {
                None
            }
        })
        .collect()
}

fn get_galaxies_with_empty_offset(sky: MilkyWay, offset: usize) -> Vec<(usize, usize)> {
    let (x_empty, y_empty) = list_empty_lines(sky.clone());

    let points =
        sky.into_iter().enumerate().flat_map(|(x, line)| {
            line.into_iter().enumerate().filter_map(move |(y, elem)| {
                if elem.is_some() {
                    Some((x, y))
                } else {
                    None
                }
            })
        });

    points
        .map(|(x, y)| {
            (
                x + (x_empty.iter().filter(|empty| x > **empty).count() * offset),
                y + (y_empty.iter().filter(|empty| y > **empty).count() * offset),
            )
        })
        .collect()
}

fn rotate(sky: MilkyWay) -> MilkyWay {
    let mut result: MilkyWay = Vec::new();
    let x_len = sky.len();
    let y_len = sky.first().unwrap().len();
    for y in 0..y_len {
        let mut line = Vec::new();
        for x in 0..x_len {
            line.push(sky.get(x).unwrap().get(y).unwrap().to_owned());
        }
        result.push(line);
    }

    result
}

fn find_manhatan_distance_for_each(points: &[(usize, usize)]) -> usize {
    points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            points[(index + 1)..]
                .iter()
                .map(|other| manhattan_distance(*point, *other))
                .sum::<usize>()
        })
        .sum()
}

fn manhattan_distance((ax, ay): (usize, usize), (bx, by): (usize, usize)) -> usize {
    ax.abs_diff(bx) + ay.abs_diff(by)
}

#[cfg(test)]
mod test {
    use super::{gold, parse::parse, silver};

    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_silver() {
        let galaxy = parse(INPUT);
        assert_eq!(silver(galaxy), 374)
    }
    #[test]
    fn test_gold() {
        let galaxy = parse(INPUT);
        assert_eq!(gold(galaxy), 8410)
    }
}
