#![allow(unused_imports)]

use std::convert::TryFrom;

use anyhow::bail;
use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

use aoc_utils::libs::*;
use aoc_utils::try_from_lines;

type Idx = i32;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Data<'a> {
    adjacent: HashMap<Idx, Vec<Idx>>,
    interned: HashMap<Idx, &'a str>,
    start_idx: Idx,
    end_idx: Idx,
}

#[aoc_generator(day12)]
pub fn generate<'input>(s: &'input str) -> Data<'input> {
    let mut data = Data::default();
    let mut inverse_interned = HashMap::default();
    let mut next_index = 0;
    let mut inc_next = || {
        next_index += 1;
        next_index
    };
    for line in s.lines() {
        let (from, to) = line.split_once('-').unwrap();

        let from_idx = *inverse_interned.entry(from).or_insert_with(&mut inc_next);
        data.interned.insert(from_idx, from);
        let to_idx = *inverse_interned.entry(to).or_insert_with(&mut inc_next);
        data.interned.insert(to_idx, to);

        data.adjacent.entry(from_idx).or_default().push(to_idx);
        data.adjacent.entry(to_idx).or_default().push(from_idx);
    }

    data.start_idx = inverse_interned["start"];
    data.end_idx = inverse_interned["end"];

    data
}

fn count_paths(data: &Data<'_>, allow_small_revisit: bool) -> i32 {
    let mut to_visit = Vec::with_capacity(data.adjacent.len());
    let mut visited = Vec::with_capacity(data.adjacent.len());
    to_visit.push((data.start_idx, 1, !allow_small_revisit));

    let mut paths_to_end = 0;

    while let Some((current, depth_then, small_revisited)) = to_visit.pop() {
        visited.truncate(depth_then);
        if current == data.end_idx {
            paths_to_end += 1;
            #[cfg(debug_assertions)]
            {
                println!(
                    "{}",
                    visited
                        .iter()
                        .map(|v| data.interned[v])
                        .chain(["end"])
                        .join(",")
                );
            }
            continue;
        }

        visited.push(current);
        let depth = visited.len();

        to_visit.extend(data.adjacent[&current].iter().rev().filter_map(|&adj| {
            if data.interned[&adj].as_bytes()[0].is_ascii_uppercase() {
                Some((adj, depth, small_revisited))
            } else if adj == data.start_idx {
                None
            } else {
                let visits = visited.iter().filter(|&&vis| vis == adj).count();

                match (small_revisited, visits) {
                    (false, 1) if allow_small_revisit => Some((adj, depth, true)),
                    (_, 0) => Some((adj, depth, small_revisited)),
                    (true, _) => None,
                    _ => unreachable!(),
                }
            }
        }));
    }

    paths_to_end
}

#[aoc(day12, part1)]
pub fn day12_part1(data: &Data<'_>) -> i32 {
    count_paths(data, false)
}

#[aoc(day12, part2)]
pub fn day12_part2(data: &Data<'_>) -> i32 {
    count_paths(data, true)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Data;

    const EXAMPLE_INPUTS: [&str; 3] = [
        "start-A\n\
             start-b\n\
             A-c\n\
             A-b\n\
             b-d\n\
             A-end\n\
             b-end",
        "dc-end\n\
             HN-start\n\
             start-kj\n\
             dc-start\n\
             dc-HN\n\
             LN-dc\n\
             HN-end\n\
             kj-sa\n\
             kj-HN\n\
             kj-dc",
        "fs-end\n\
             he-DX\n\
             fs-he\n\
             start-DX\n\
             pj-DX\n\
             end-zg\n\
             zg-sl\n\
             zg-pj\n\
             pj-he\n\
             RW-he\n\
             fs-DX\n\
             pj-RW\n\
             zg-RW\n\
             start-pj\n\
             he-WI\n\
             zg-he\n\
             pj-fs\n\
             start-RW",
    ];

    fn get_example_data() -> [Data<'static>; 3] {
        EXAMPLE_INPUTS.map(super::generate)
    }

    #[test]
    fn day12_part1() {
        for (input, data) in get_example_data().iter().zip([10, 19, 226]) {
            assert_eq!(super::day12_part1(input), data);
        }
    }

    #[test]
    fn day12_part2() {
        for (input, data) in get_example_data().iter().zip([36, 103, 3509]) {
            assert_eq!(super::day12_part2(input), data);
        }
    }
}
