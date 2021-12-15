#![allow(unused_imports)]

use std::cmp::Reverse;
use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::BinaryHeap;
use std::convert::TryFrom;

use anyhow::bail;
use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use aoc_utils::grids::for_neighbours_4;
use aoc_utils::libs::*;
use aoc_utils::structs::PriorityValue;
use aoc_utils::try_from_lines;

pub type Data<'a> = &'a str;

#[aoc_generator(day15)]
pub fn generate<'input>(s: &'input str) -> Vec<Data<'input>> {
    s.lines().collect()
}

fn shortest_path<const REPEAT: usize>(data: &[Data<'_>]) -> i32 {
    let (orig_w, orig_h) = (data[0].len(), data.len());
    let wh = (orig_w * REPEAT, orig_h * REPEAT);
    let (w, h) = wh;

    let mut scores = HashMap::default();
    scores.insert((0, 0), 0);
    let mut visited = HashSet::default();

    let mut visit_next = BinaryHeap::new();
    visit_next.push(PriorityValue {
        priority: Reverse(0),
        value: (0, 0),
    });

    while let Some(current) = visit_next.pop() {
        let PriorityValue {
            priority: Reverse(current_score),
            value: current,
        } = current;
        if visited.contains(&current) {
            continue;
        }

        for_neighbours_4(current, wh, |next| {
            if visited.contains(&next) {
                return;
            }
            let next_cost = (data[next.1 % orig_h].as_bytes()[next.0 % orig_w] - b'0') as usize;
            let next_cost = (next_cost + (next.1 / orig_h) + (next.0 / orig_w) - 1) as i32 % 9 + 1;
            let next_score = current_score + next_cost;
            match scores.entry(next) {
                Occupied(ent) => {
                    if next_score < *ent.get() {
                        *ent.into_mut() = next_score;
                        visit_next.push(PriorityValue {
                            priority: Reverse(next_score),
                            value: next,
                        });
                    }
                }
                Vacant(ent) => {
                    ent.insert(next_score);
                    visit_next.push(PriorityValue {
                        priority: Reverse(next_score),
                        value: next,
                    });
                }
            }
        });
        visited.insert(current);
        if current == (w - 1, h - 1) {
            return scores[&current];
        }
    }

    unreachable!()
}

#[aoc(day15, part1)]
pub fn day15_part1(data: &[Data<'_>]) -> i32 {
    shortest_path::<1>(data)
}

#[aoc(day15, part2)]
pub fn day15_part2(data: &[Data<'_>]) -> i32 {
    shortest_path::<5>(data)
}

#[cfg(test)]
mod tests {
    use super::Data;

    const EXAMPLE_INPUT: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;

    const EXAMPLE_INPUT2: &str = r#"19999
19999
19111
19191
11191"#;

    fn get_example_data() -> Vec<Data<'static>> {
        vec![
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581",
        ]
    }

    fn get_example_data2() -> Vec<Data<'static>> {
        vec![
            "19999",
            "19999",
            "19111",
            "19191",
            "11191",
        ]
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
        assert_eq!(super::generate(EXAMPLE_INPUT2), get_example_data2());
    }

    #[test]
    fn day15_part1() {
        assert_eq!(super::day15_part1(&get_example_data()), 40);
        assert_eq!(super::day15_part1(&get_example_data2()), 12);
    }

    #[test]
    fn day15_part2() {
        assert_eq!(super::day15_part2(&get_example_data()), 315);
    }
}
