#![allow(clippy::naive_bytecount)]

use super::{aoc, aoc_generator};
use aoc_utils::try_from_lines;
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
pub struct Line {
    value1: u8,
    value2: u8,
    letter: u8,
    password: Vec<u8>,
}

impl TryFrom<&str> for Line {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut s = s.as_bytes().iter();
        let value1 = parse_bound(&mut s, b'-')?;
        let value2 = parse_bound(&mut s, b' ')?;
        let letter = *s.next().ok_or(())?;
        s.next().ok_or(())?;
        s.next().ok_or(())?; // b": "
        let password = s.as_slice().to_owned();

        fn parse_bound<'e>(
            iter: &mut impl Iterator<Item = &'e u8>,
            stop_char: u8,
        ) -> Result<u8, ()> {
            let mut value = 0;
            for b in iter {
                match b {
                    v @ b'0'..=b'9' => {
                        value = value * 10 + (v - b'0');
                    }
                    &c if c == stop_char => break,
                    _ => return Err(()),
                }
            }
            Ok(value)
        }

        Ok(Line {
            value1,
            value2,
            letter,
            password,
        })
    }
}

#[aoc_generator(day2)]
pub fn generate(s: &str) -> Vec<Line> {
    try_from_lines(s).expect("couldn't parse input")
}

#[aoc(day2, part1, filter_count)]
pub fn day2_part1_filter_count(values: &[Line]) -> i32 {
    values
        .iter()
        .filter(|line| {
            let count = line.password.iter().filter(|&&c| c == line.letter).count();
            (line.value1..=line.value2).contains(&count.try_into().expect("letter count too large"))
        })
        .count()
        .try_into()
        .expect("valid password count too large")
}

#[aoc(day2, part1, map_sum)]
pub fn day2_part1_map_sum(values: &[Line]) -> i32 {
    values
        .iter()
        .map(|line| {
            let count = line
                .password
                .iter()
                .map(|&c| (c == line.letter) as u8)
                .sum();
            (line.value1..=line.value2).contains(&count) as i32
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn day2_part2(values: &[Line]) -> i32 {
    values
        .iter()
        .filter(|line| {
            (line.password[(line.value1 - 1) as usize] == line.letter)
                ^ (line.password[(line.value2 - 1) as usize] == line.letter)
        })
        .count()
        .try_into()
        .expect("valid password count too large")
}

#[cfg(test)]
mod tests {
    use super::Line;

    const EXAMPLE_INPUT: &str = r"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    fn get_example_data() -> Vec<Line> {
        vec![
            Line {
                value1: 1,
                value2: 3,
                letter: b'a',
                password: "abcde".as_bytes().to_owned(),
            },
            Line {
                value1: 1,
                value2: 3,
                letter: b'b',
                password: "cdefg".as_bytes().to_owned(),
            },
            Line {
                value1: 2,
                value2: 9,
                letter: b'c',
                password: "ccccccccc".as_bytes().to_owned(),
            },
        ]
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day2_part1_filter_count() {
        assert_eq!(super::day2_part1_filter_count(&get_example_data()), 2);
    }

    #[test]
    fn day2_part1_map_sum() {
        assert_eq!(super::day2_part1_map_sum(&get_example_data()), 2);
    }

    #[test]
    fn day2_part2() {
        assert_eq!(super::day2_part2(&get_example_data()), 1);
    }
}
