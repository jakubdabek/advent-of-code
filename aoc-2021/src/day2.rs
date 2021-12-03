use std::convert::TryFrom;

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

use anyhow::bail;
use aoc_utils::libs::*;
use aoc_utils::try_from_lines;

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
pub enum Movement {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl TryFrom<&'_ str> for Movement {
    type Error = anyhow::Error;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let (command, distance) = line.split_once(' ').context("split_once")?;
        let distance = lexical::parse(distance)?;

        let movement = match command {
            "forward" => Movement::Forward(distance),
            "down" => Movement::Down(distance),
            "up" => Movement::Up(distance),
            _ => bail!("invalid movement"),
        };

        Ok(movement)
    }
}

#[aoc_generator(day2)]
pub fn generate(s: &str) -> Vec<Movement> {
    try_from_lines(s).expect("couldn't parse input")
}

#[aoc(day2, part1)]
pub fn day2_part1(values: &[Movement]) -> i32 {
    use Movement::*;
    let mut depth = 0;
    let mut horizontal = 0;
    for value in values {
        match value {
            Forward(f) => horizontal += f,
            Down(d) => depth += d,
            Up(u) => depth -= u,
        }
    }

    depth * horizontal
}

#[aoc(day2, part2)]
pub fn day2_part2(values: &[Movement]) -> i32 {
    use Movement::*;
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for value in values {
        match value {
            Forward(f) => {
                horizontal += f;
                depth += f * aim;
            }
            Down(d) => aim += d,
            Up(u) => aim -= u,
        }
    }

    depth * horizontal
}

#[cfg(test)]
mod tests {
    use super::Movement;

    const EXAMPLE_INPUT: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

    fn get_example_data() -> Vec<Movement> {
        use Movement::*;
        vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2)]
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day2_part1() {
        assert_eq!(super::day2_part1(&get_example_data()), 150);
    }

    #[test]
    fn day2_part2() {
        assert_eq!(super::day2_part2(&get_example_data()), 900);
    }
}
