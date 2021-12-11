#![allow(unused_imports)]

use std::collections::VecDeque;
use std::convert::TryFrom;

use anyhow::bail;
use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use aoc_utils::grids::{for_neighbours_8, position_in_grid, print_digit, print_grid};
use aoc_utils::libs::*;
use aoc_utils::try_from_lines;

pub type Data = Vec<u8>;

#[aoc_generator(day11)]
pub fn generate(s: &str) -> Vec<Data> {
    s.lines()
        .map(|line| line.bytes().map(|c| c - b'0').collect())
        .collect()
}

fn simulate_flashes(data: &[Data], stop_after_synchronize: bool) -> i32 {
    let mut data = data.to_owned();
    let wh = (data[0].len(), data.len());

    let mut queue = VecDeque::with_capacity(32);
    let mut flashes = 0;

    let mut iter = 0;
    while stop_after_synchronize || iter < 100 {
        #[cfg(debug_assertions)]
        if iter % 10 == 0 {
            println!("after {}", iter);
            print_grid(&data, print_digit);
            println!();
        }
        // increment all by 1
        data.iter_mut().flatten().for_each(|d| match d {
            d @ 0..=9 => *d += 1,
            _ => unreachable!(),
        });

        // flash, charge adjacent, flash etc.
        assert!(queue.is_empty());
        while let Some(xy) = position_in_grid(&data, |&d| d == 10) {
            queue.push_back(xy);
            while let Some((x, y)) = queue.pop_front() {
                match &mut data[y][x] {
                    power @ 10 => {
                        for_neighbours_8((x, y), wh, |pos| queue.push_back(pos));
                        flashes += 1;
                        *power = 0;
                    }
                    0 => {}
                    power @ 1..=9 => *power += 1,
                    _ => unreachable!(),
                }
            }
        }

        iter += 1;
        if stop_after_synchronize && data.iter().flatten().all(|&d| d == 0) {
            return iter;
        }
    }

    if stop_after_synchronize {
        unreachable!("flashes didn't synchronize!");
    }
    flashes
}

#[aoc(day11, part1)]
pub fn day11_part1(data: &[Data]) -> i32 {
    simulate_flashes(data, false)
}

#[aoc(day11, part2)]
pub fn day11_part2(data: &[Data]) -> i32 {
    simulate_flashes(data, true)
}

#[cfg(test)]
mod tests {
    use super::Data;

    const EXAMPLE_INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

    fn get_example_data() -> Vec<Data> {
        vec![
            vec![5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
            vec![2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
            vec![5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
            vec![6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
            vec![6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
            vec![4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
            vec![2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
            vec![6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
            vec![4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
            vec![5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
        ]
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day11_part1() {
        assert_eq!(super::day11_part1(&get_example_data()), 1656);
    }

    #[test]
    fn day11_part2() {
        assert_eq!(super::day11_part2(&get_example_data()), 195);
    }
}
