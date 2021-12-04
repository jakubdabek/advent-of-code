#![allow(unused_imports)]

use std::collections::HashMap;
use std::convert::TryFrom;

use anyhow::bail;
use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};

use aoc_utils::libs::itertools::Itertools;
use aoc_utils::libs::*;
use aoc_utils::try_from_lines;

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub struct Board {
    numbers: [[u8; 5]; 5],
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub struct Data {
    drawn: Vec<u8>,
    boards: Vec<Board>,
}

#[aoc_generator(day4)]
pub fn generate(s: &str) -> Data {
    let (drawn, boards) = s.split_once("\n\n").unwrap();
    let drawn = drawn.split(',').map(|n| n.parse().unwrap()).collect_vec();

    let boards = boards
        .split("\n\n")
        .map(|board_str| {
            let mut numbers = [[0; 5]; 5];

            board_str
                .split_ascii_whitespace()
                .zip(numbers.iter_mut().flatten())
                .for_each(|(s, n)| *n = s.parse().unwrap());

            Board { numbers }
        })
        .collect();

    Data { boards, drawn }
}

fn winning_value(when_drawn: &[usize; 100], board: &Board) -> (i32, usize) {
    let mut when_row = [0; 5];
    let mut when_column = [0; 5];

    for (i, row) in board.numbers.iter().enumerate() {
        for (j, &v) in row.iter().enumerate() {
            let v_drawn = when_drawn[v as usize];
            when_row[i] = when_row[i].max(v_drawn);
            when_column[j] = when_column[j].max(v_drawn);
        }
    }

    let when_won = when_row
        .iter()
        .chain(when_column.iter())
        .copied()
        .min()
        .unwrap();

    let value = board
        .numbers
        .iter()
        .flatten()
        .copied()
        .filter(|&v| when_drawn[v as usize] > when_won)
        // might overflow on `u8`
        .map(|v| v as i32)
        .sum();

    (value, when_won)
}

fn first_last_winning_value(data: &Data) -> (i32, i32) {
    let mut when_drawn = [usize::MAX; 100];
    for (i, &d) in data.drawn.iter().enumerate() {
        when_drawn[d as usize] = i;
    }

    let (first, last) = data
        .boards
        .iter()
        .map(|board| winning_value(&when_drawn, board))
        .minmax_by_key(|&(_, when)| when)
        .into_option()
        .unwrap();

    let value = |(winning_board_value, when_won)| winning_board_value * data.drawn[when_won] as i32;

    (value(first), value(last))
}

#[aoc(day4, part1)]
pub fn day4_part1(data: &Data) -> i32 {
    first_last_winning_value(data).0
}

#[aoc(day4, part2)]
pub fn day4_part2(data: &Data) -> i32 {
    first_last_winning_value(data).1
}

#[cfg(test)]
mod tests {
    use super::{Board, Data};

    const EXAMPLE_INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    fn get_example_data() -> Data {
        Data {
            drawn: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            boards: vec![
                Board {
                    numbers: [
                        [22, 13, 17, 11, 0],
                        [8, 2, 23, 4, 24],
                        [21, 9, 14, 16, 7],
                        [6, 10, 3, 18, 5],
                        [1, 12, 20, 15, 19],
                    ],
                },
                Board {
                    numbers: [
                        [3, 15, 0, 2, 22],
                        [9, 18, 13, 17, 5],
                        [19, 8, 7, 25, 23],
                        [20, 11, 10, 24, 4],
                        [14, 21, 16, 12, 6],
                    ],
                },
                Board {
                    numbers: [
                        [14, 21, 17, 24, 4],
                        [10, 16, 15, 9, 19],
                        [18, 8, 23, 26, 20],
                        [22, 11, 13, 6, 5],
                        [2, 0, 12, 3, 7],
                    ],
                },
            ],
        }
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day4_part1() {
        assert_eq!(super::day4_part1(&get_example_data()), 4512);
    }

    #[test]
    fn day4_part2() {
        assert_eq!(super::day4_part2(&get_example_data()), 1924);
    }
}
