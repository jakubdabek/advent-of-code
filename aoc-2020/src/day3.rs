use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::try_from_lines;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Field {
    Empty,
    Tree,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    slope: Vec<Field>,
}

impl TryFrom<&str> for Line {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        s.as_bytes()
            .iter()
            .map(|c| match c {
                b'.' => Ok(Field::Empty),
                b'#' => Ok(Field::Tree),
                _ => Err(()),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|fields| Line { slope: fields })
    }
}

#[aoc_generator(day3)]
pub fn generate(s: &str) -> Vec<Line> {
    try_from_lines(s).expect("couldn't parse input")
}

fn count_slope_trees(lines: &[Line], dx: isize, dy: isize) -> usize {
    lines
        .iter()
        .enumerate()
        .filter(|&(y, line)| {
            y as isize % dy == 0
                && line.slope[((y as isize * dx / dy) % line.slope.len() as isize) as usize]
                    == Field::Tree
        })
        .count()
}

#[aoc(day3, part1)]
pub fn day3_part1(lines: &[Line]) -> usize {
    count_slope_trees(lines, 3, 1)
}

#[aoc(day3, part2)]
pub fn day3_part2(lines: &[Line]) -> usize {
    let params = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    params
        .iter()
        .map(|&(dx, dy)| count_slope_trees(lines, dx, dy))
        .product()
}

#[cfg(test)]
mod tests {
    use super::Line;

    const EXAMPLE_INPUT: &str = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    fn get_example_data() -> Vec<Line> {
        use super::Field::{Empty as E, Tree as T};
        vec![
            Line {
                slope: vec![E, E, T, T, E, E, E, E, E, E, E],
            },
            Line {
                slope: vec![T, E, E, E, T, E, E, E, T, E, E],
            },
            Line {
                slope: vec![E, T, E, E, E, E, T, E, E, T, E],
            },
            Line {
                slope: vec![E, E, T, E, T, E, E, E, T, E, T],
            },
            Line {
                slope: vec![E, T, E, E, E, T, T, E, E, T, E],
            },
            Line {
                slope: vec![E, E, T, E, T, T, E, E, E, E, E],
            },
            Line {
                slope: vec![E, T, E, T, E, T, E, E, E, E, T],
            },
            Line {
                slope: vec![E, T, E, E, E, E, E, E, E, E, T],
            },
            Line {
                slope: vec![T, E, T, T, E, E, E, T, E, E, E],
            },
            Line {
                slope: vec![T, E, E, E, T, T, E, E, E, E, T],
            },
            Line {
                slope: vec![E, T, E, E, T, E, E, E, T, E, T],
            },
        ]
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day3_part1() {
        assert_eq!(super::day3_part1(&get_example_data()), 7);
    }

    #[test]
    fn day3_part2() {
        assert_eq!(super::day3_part2(&get_example_data()), 336);
    }
}
