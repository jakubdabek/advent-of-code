#![allow(unused_imports)]

use std::convert::TryFrom;
use std::ops::RangeInclusive;

use anyhow::bail;
use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use aoc_utils::libs::*;
use aoc_utils::{try_from_lines, Ext};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data {
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
}

#[aoc_generator(day17)]
pub fn generate(s: &str) -> Data {
    let (_, coords) = s.split_once(": ").unwrap();
    let (x, y) = coords.split_once(", ").unwrap();
    let [x, y] = [x, y].map(|coord| {
        let (start, end) = coord[2..].split_once("..").unwrap();
        let [start, end] = [start, end].map(|n| lexical::parse(n).unwrap());
        start..=end
    });

    Data { x, y }
}

#[aoc(day17, part1)]
pub fn day17_part1(data: &Data) -> i32 {
    (-2000..0)
        .find_map(|vy0| {
            let mut vy = vy0;
            std::iter::successors(Some(0), |y| {
                vy -= 1;
                Some(y + vy + 1)
            })
            .take_while(|y| y >= data.y.start())
            .find(|y| y <= data.y.end())
            .map(|_| (vy0.pow(2) + vy0) / 2)
        })
        .unwrap()
}

#[aoc(day17, part2)]
pub fn day17_part2(data: &Data) -> i32 {
    (-500..500)
        .flat_map(|vy0| (-500..=500).map(move |vx0: i32| (vx0, vy0)))
        .filter_map(|(vx0, vy0)| {
            let (mut vx, mut vy) = (vx0, vy0);
            std::iter::successors(Some((0, 0)), |(x, y)| {
                let x = x + vx;
                let y = y + vy;
                vy -= 1;
                vx -= vx.signum();

                Some((x, y))
            })
            .take_while(|(x, y)| {
                y >= data.y.start()
                    && if data.x.start() < &0 {
                        x >= data.x.start()
                    } else {
                        x <= data.x.end()
                    }
            })
            .find(|(x, y)| {
                y <= data.y.end()
                    && if data.x.start() < &0 {
                        x <= data.x.end()
                    } else {
                        x >= data.x.start()
                    }
            })
            .map(|_| (vx0, vy0))
        })
        .count() as _
}

#[cfg(test)]
mod tests {
    use super::Data;

    const EXAMPLE_INPUT: &str = r#"target area: x=20..30, y=-10..-5"#;

    fn get_example_data() -> Data {
        Data {
            x: 20..=30,
            y: -10..=-5,
        }
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day17_part1() {
        assert_eq!(super::day17_part1(&get_example_data()), 45);
    }

    #[test]
    fn day17_part2() {
        assert_eq!(super::day17_part2(&get_example_data()), 112);
    }
}
