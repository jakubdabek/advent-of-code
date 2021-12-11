#![allow(unused_imports)]

use std::convert::TryFrom;

use anyhow::bail;
use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use aoc_utils::libs::*;
use aoc_utils::try_from_lines;

pub type Data<'a> = &'a str;

#[aoc_generator(day10)]
pub fn generate<'input>(s: &'input str) -> Vec<Data<'input>> {
    s.lines().collect()
}

fn invalid_score(c: u8) -> i64 {
    match c {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => unreachable!(),
    }
}

fn completion_score(c: u8) -> i64 {
    match c {
        b')' => 1,
        b']' => 2,
        b'}' => 3,
        b'>' => 4,
        _ => unreachable!(),
    }
}

#[aoc(day10, part1)]
pub fn day10_part1(data: &[Data<'_>]) -> i64 {
    let mut stack = Vec::with_capacity(data[0].len());
    data.iter()
        .map(|data| {
            stack.clear();
            for c in data.bytes() {
                match c {
                    b'[' => stack.push(b']'),
                    b'(' => stack.push(b')'),
                    b'{' => stack.push(b'}'),
                    b'<' => stack.push(b'>'),
                    c => match stack.pop() {
                        Some(end) if c == end => {}
                        Some(_) => return invalid_score(c),
                        None => unreachable!(),
                    },
                }
            }
            0
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn day10_part2(data: &[Data<'_>]) -> i64 {
    let mut stack = Vec::with_capacity(data[0].len());
    let mut completion_scores = data
        .iter()
        .filter_map(|data| {
            stack.clear();
            for c in data.bytes() {
                match c {
                    b'[' => stack.push(b']'),
                    b'(' => stack.push(b')'),
                    b'{' => stack.push(b'}'),
                    b'<' => stack.push(b'>'),
                    c => match stack.pop() {
                        Some(end) if c == end => {}
                        Some(_) => return None,
                        None => unreachable!(),
                    },
                }
            }
            Some(
                stack
                    .iter()
                    .rev()
                    .fold(0, |acc, &end| acc * 5 + completion_score(end)),
            )
        })
        .collect_vec();

    let center = completion_scores.len() / 2;
    // println!("{}/2 = {}", completion_scores.len(), center);
    // println!("{:?}", completion_scores);
    *completion_scores.select_nth_unstable(center).1
}

#[cfg(test)]
mod tests {
    use super::Data;

    const EXAMPLE_INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

    fn get_example_data() -> Vec<Data<'static>> {
        vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ]
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day10_part1() {
        assert_eq!(super::day10_part1(&get_example_data()), 26397);
    }

    #[test]
    fn day10_part2() {
        assert_eq!(super::day10_part2(&get_example_data()), 288957);
    }
}
