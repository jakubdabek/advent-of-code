use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::try_from_lines;
use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
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
            let count = line.password.iter().map(|&c| (c == line.letter) as u8).sum();
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
