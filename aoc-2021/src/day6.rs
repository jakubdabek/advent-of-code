#![allow(unused_imports)]

use std::convert::TryFrom;

use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use anyhow::bail;
use aoc_utils::libs::*;
use aoc_utils::try_from_lines;

type Data = u8;

#[aoc_generator(day6)]
pub fn generate(s: &str) -> Vec<Data> {
    s.split(',').map(str::parse).try_collect().unwrap()
}

fn population_after(days: usize, data: &[Data]) -> u64 {
    let mut population = [0; 9];
    for &d in data {
        population[d as usize] += 1;
    }

    for _ in 0..days {
        population.rotate_left(1);
        population[6] += population[8];
    }

    population.iter().sum()
}

#[aoc(day6, part1)]
pub fn day6_part1(data: &[Data]) -> u64 {
    population_after(80, data)
}

#[aoc(day6, part2)]
pub fn day6_part2(data: &[Data]) -> u64 {
    population_after(256, data)
}

#[cfg(test)]
mod tests {
    use super::Data;

    const EXAMPLE_INPUT: &str = r#"3,4,3,1,2"#;

    fn get_example_data() -> Vec<Data> {
        vec![3, 4, 3, 1, 2]
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day6_part1() {
        assert_eq!(super::day6_part1(&get_example_data()), 5934);
    }

    #[test]
    fn day6_part2() {
        assert_eq!(super::day6_part2(&get_example_data()), 26984457539);
    }
}
