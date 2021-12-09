#![allow(unused_imports)]

use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
use std::convert::TryFrom;

use anyhow::bail;
use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::izip;
use itertools::Itertools;

use aoc_utils::libs::*;
use aoc_utils::{for_neighbours, try_from_lines};

pub type Data<'a> = &'a str;

#[aoc_generator(day9)]
pub fn generate<'input>(s: &'input str) -> Vec<Data<'input>> {
    s.lines().collect()
}

const PADDING: &[u8] = &[u8::MAX; 200];

#[aoc(day9, part1)]
pub fn day9_part1(data: &[Data<'_>]) -> i32 {
    [PADDING]
        .into_iter()
        .chain(data.iter().map(|s| s.as_bytes()))
        .chain([PADDING])
        .tuple_windows()
        .map(|(win0, win1, win2)| {
            izip!(
                win0.iter().copied(),
                [u8::MAX]
                    .into_iter()
                    .chain(win1.iter().copied())
                    .chain([u8::MAX])
                    .tuple_windows(),
                win2.iter().copied(),
            )
            .filter(|&(a, (x, y, z), c)| y < x && y < z && y < a && y < c)
            .map(|(_, b, _)| (b.1 - b'0') + 1)
            .sum::<u8>() as i32
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq)]
enum State {
    Initial,
    Visited(u16),
    Wall,
}

fn _print_state<I>(state: &[I])
where
    for<'a> &'a I: IntoIterator<Item = &'a State>,
{
    for row in state {
        for s in row {
            match s {
                State::Initial => print!("."),
                State::Visited(x) => print!("{}", x),
                State::Wall => print!("#"),
            }
        }
        println!()
    }
}

#[aoc(day9, part2)]
pub fn day9_part2(data: &[Data<'_>]) -> i32 {
    let width = data[0].len();
    let height = data.len();
    let mut state = data
        .iter()
        .map(|_| vec![State::Initial; width])
        .collect_vec();

    let mut queue = VecDeque::with_capacity(32);

    queue.push_back((0, 0));

    let mut basins = vec![];
    let mut visited_rows = 0;

    loop {
        if !matches!(basins.last(), Some(0)) {
            basins.push(0);
        }
        let current_basin_index = basins.len();
        let current_basin = basins.last_mut().unwrap();
        while let Some(current) = queue.pop_front() {
            let (x, y) = current;
            let current_state = &mut state[y][x];
            let current_depth = data[y].as_bytes()[x] - b'0';
            match (*current_state, current_depth) {
                (State::Initial, 9) => {
                    *current_state = State::Wall;
                    continue;
                }
                (State::Initial, _) => {
                    *current_basin += 1;
                    *current_state = State::Visited(current_basin_index as _);
                }
                (State::Visited(b), _) => {
                    debug_assert_eq!(b as usize, current_basin_index);
                    continue;
                }
                (State::Wall, 9) => {
                    continue;
                }
                _ => unreachable!(),
            }
            for_neighbours(current, (width, height), |next| {
                queue.push_back(next);
            });
        }

        // _print_state(&state);

        if let Some(unvisited_position) =
            state[visited_rows..]
                .iter()
                .zip(visited_rows..)
                .find_map(|(row, y)| {
                    row.iter()
                        .position(|s| s == &State::Initial)
                        .map(|x| (x, y))
                })
        {
            visited_rows = unvisited_position.1;
            queue.push_back(unvisited_position);
        } else {
            break;
        }
    }

    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::Data;

    const EXAMPLE_INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    fn get_example_data() -> Vec<Data<'static>> {
        vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ]
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day9_part1() {
        assert_eq!(super::day9_part1(&get_example_data()), 15);
    }

    #[test]
    fn day9_part2() {
        assert_eq!(super::day9_part2(&get_example_data()), 1134);
    }
}
