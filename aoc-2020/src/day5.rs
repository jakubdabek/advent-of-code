use super::{aoc, aoc_generator};
use aoc_utils::try_from_lines;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BinaryPartition {
    Lower,
    Upper,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    row_partitions: Vec<BinaryPartition>,
    col_partitions: Vec<BinaryPartition>,
}

impl TryFrom<&str> for Line {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let s = s.as_bytes();
        let row_partitions = s
            .get(..7)
            .ok_or(())?
            .iter()
            .map(|c| match c {
                b'F' => Ok(BinaryPartition::Lower),
                b'B' => Ok(BinaryPartition::Upper),
                _ => Err(()),
            })
            .collect::<Result<Vec<_>, _>>()?;
        let col_partitions = s
            .get(7..10)
            .ok_or(())?
            .iter()
            .map(|c| match c {
                b'L' => Ok(BinaryPartition::Lower),
                b'R' => Ok(BinaryPartition::Upper),
                _ => Err(()),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Line {
            row_partitions,
            col_partitions,
        })
    }
}

#[aoc_generator(day5)]
pub fn generate(s: &str) -> Vec<Line> {
    try_from_lines(s).expect("couldn't parse input")
}

fn partition(
    mut lower_bound: usize,
    mut upper_bound: usize,
    partitions: &[BinaryPartition],
) -> Option<usize> {
    for part in partitions {
        match part {
            BinaryPartition::Lower => upper_bound = (lower_bound + upper_bound) / 2,
            BinaryPartition::Upper => lower_bound = (lower_bound + upper_bound) / 2,
        }
    }

    if lower_bound + 1 == upper_bound {
        Some(lower_bound)
    } else {
        None
    }
}

fn seat_id(line: &Line) -> usize {
    let row_id = partition(0, 128, &line.row_partitions).unwrap();
    let col_id = partition(0, 8, &line.col_partitions).unwrap();
    row_id * 8 + col_id
}

#[aoc(day5, part1)]
pub fn day5_part1(lines: &[Line]) -> usize {
    lines.iter().map(seat_id).max().unwrap()
}

#[aoc(day5, part2)]
pub fn day5_part2(lines: &[Line]) -> usize {
    use aoc_utils::libs::itertools::Itertools;
    lines
        .iter()
        .map(seat_id)
        .sorted()
        .tuple_windows()
        .find_map(|(a, b)| if a + 1 != b { Some(a + 1) } else { None })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    const EXAMPLE_DATA: &str = r"FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    #[test]
    fn seat_id() {
        assert_eq!(super::seat_id(&"FBFBBFFRLR".try_into().unwrap()), 357);
        assert_eq!(super::seat_id(&"BFFFBBFRRR".try_into().unwrap()), 567);
        assert_eq!(super::seat_id(&"FFFBBBFRRR".try_into().unwrap()), 119);
        assert_eq!(super::seat_id(&"BBFFBBFRLL".try_into().unwrap()), 820);
    }

    #[test]
    fn day5_part1() {
        assert_eq!(super::day5_part1(&super::generate(EXAMPLE_DATA)), 820);
    }
}
