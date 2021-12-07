#![allow(unused_imports)]

use std::convert::TryFrom;

use anyhow::bail;
use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use aoc_utils::libs::*;
use aoc_utils::try_from_lines;

type Data = i16;

#[aoc_generator(day7)]
pub fn generate(s: &str) -> Vec<Data> {
    s.split(',').map(str::parse).try_collect().unwrap()
}

#[aoc(day7, part1)]
pub fn day7_part1(data: &[Data]) -> i32 {
    let (min, max) = data.iter().copied().minmax().into_option().unwrap();
    let mut fuel = vec![0; (max - min) as usize];
    for &d in data {
        for (f, i) in fuel.iter_mut().zip(min..) {
            *f += (d - i).abs() as i32;
        }
    }

    fuel.iter().copied().min().unwrap()
}

#[aoc(day7, part2)]
pub fn day7_part2(data: &[Data]) -> i32 {
    let (min, max) = data.iter().copied().minmax().into_option().unwrap();
    let mut fuel = vec![0; (max - min) as usize];
    for &d in data {
        for (f, i) in fuel.iter_mut().zip(min..) {
            let diff = (d - i).abs() as i32;
            *f += diff * (diff + 1) / 2;
        }
    }

    fuel.iter().copied().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::Data;

    const EXAMPLE_INPUT: &str = r#"16,1,2,0,4,2,7,1,2,14"#;

    fn get_example_data() -> Vec<Data> {
        vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day7_part1() {
        assert_eq!(super::day7_part1(&get_example_data()), 37);
    }

    #[test]
    fn day7_part2() {
        assert_eq!(super::day7_part2(&get_example_data()), 168);
    }
}
