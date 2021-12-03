use super::{aoc, aoc_generator};
use aoc_utils::try_from_lines;
use aoc_utils::libs::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::cell::Cell;
use std::collections::HashMap;
use std::convert::TryFrom;

type Color = String;

#[derive(Debug, Clone, PartialEq)]
pub struct ContaineeData {
    color: Color,
    capacity: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContainerData {
    color: Color,
    capacity: u32,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    container: Color,
    containees: Vec<ContaineeData>,
}

impl TryFrom<&str> for Line {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        static WHOLE_PATTERN: Lazy<Regex> = Lazy::new(|| {
            Regex::new(
                r"(?P<container>[\w ]+) bags contain (no other bags|(?P<containees>[\w ,]+))\.",
            )
            .unwrap()
        });
        static CONTAINEES_PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(?P<capacity>\d) (?P<color>[\w ]+) bags?").unwrap());

        let m = WHOLE_PATTERN.captures(s).ok_or(())?;
        let container = m.name("container").unwrap().as_str().to_owned();
        let containees = if let Some(containees) = m.name("containees") {
            CONTAINEES_PATTERN
                .captures_iter(containees.as_str())
                .map(|containee| {
                    let capacity = containee
                        .name("capacity")
                        .map(|m| m.as_str())
                        .and_then(|s| s.chars().next())
                        .and_then(|c| c.to_digit(10))
                        .unwrap();
                    let color = containee.name("color").unwrap().as_str().to_owned();

                    ContaineeData { color, capacity }
                })
                .collect()
        } else {
            vec![]
        };

        Ok(Line {
            container,
            containees,
        })
    }
}

#[aoc_generator(day7)]
pub fn generate(s: &str) -> Vec<Line> {
    try_from_lines(s).expect("couldn't parse input")
}

type ContaineeContainerGraph = HashMap<Color, (Cell<bool>, Vec<ContainerData>)>;

fn create_containee_container_graph(lines: &[Line]) -> ContaineeContainerGraph {
    let mut graph = Default::default();
    if false {
        return graph; // type inference
    }

    for Line {
        container,
        containees,
    } in lines
    {
        let mut current_can_contain = graph.entry(container.clone()).or_default().0.get();
        for ContaineeData { color, capacity } in containees {
            let (containee_can_contain, containee_containers) =
                graph.entry(color.clone()).or_default();
            if containee_can_contain.get() {
                current_can_contain = true;
            }
            containee_containers.push(ContainerData {
                color: container.clone(),
                capacity: *capacity,
            })
        }
        fill_containment_dfs(
            &graph,
            container,
            current_can_contain || container == "shiny gold",
        );

        // println!("{:#?}", graph);
    }

    fn fill_containment_dfs(
        graph: &ContaineeContainerGraph,
        color: &str,
        current_can_contain: bool,
    ) {
        let (can_contain, containers) = &graph[color];
        if current_can_contain {
            can_contain.set(true);
        }
        for container in containers {
            fill_containment_dfs(
                graph,
                &container.color,
                current_can_contain || can_contain.get(),
            );
        }
    }

    // println!("{:#?}", graph);

    graph
}

type ContainerContaineeGraph = HashMap<Color, Vec<ContaineeData>>;

fn create_container_containee_graph(lines: &[Line]) -> ContainerContaineeGraph {
    lines
        .iter()
        .map(
            |Line {
                 container,
                 containees,
             }| (container.clone(), containees.clone()),
        )
        .collect()
}

#[aoc(day7, part1)]
pub fn day7_part1(lines: &[Line]) -> usize {
    let graph = create_containee_container_graph(lines);
    graph
        .iter()
        .filter(|&(color, (can_contain_shiny_gold, _))| {
            color != "shiny gold" && can_contain_shiny_gold.get()
        })
        .count()
}

#[aoc(day7, part2)]
pub fn day7_part2(lines: &[Line]) -> usize {
    let graph = create_container_containee_graph(lines);
    fn count_dfs(graph: &ContainerContaineeGraph, container: &str) -> usize {
        graph[container]
            .iter()
            .map(|ContaineeData { color, capacity }| {
                *capacity as usize * (count_dfs(graph, color) + 1)
            })
            .sum()
    }

    count_dfs(&graph, "shiny gold")
}

#[cfg(test)]
mod tests {
    use super::{ContaineeData, Line};
    use super::itertools::Itertools;

    const EXAMPLE_INPUT: &str = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    const EXAMPLE_INPUT2: &str = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

    fn get_example_data() -> Vec<Line> {
        vec![
            Line {
                container: "dark olive".to_owned(),
                containees: vec![
                    ContaineeData {
                        color: "faded blue".to_owned(),
                        capacity: 3,
                    },
                    ContaineeData {
                        color: "dotted black".to_owned(),
                        capacity: 4,
                    },
                ],
            },
            Line {
                container: "vibrant plum".to_owned(),
                containees: vec![
                    ContaineeData {
                        color: "faded blue".to_owned(),
                        capacity: 5,
                    },
                    ContaineeData {
                        color: "dotted black".to_owned(),
                        capacity: 6,
                    },
                ],
            },
            Line {
                container: "faded blue".to_owned(),
                containees: vec![],
            },
            Line {
                container: "dotted black".to_owned(),
                containees: vec![],
            },
        ]
    }

    #[test]
    fn generate() {
        assert_eq!(
            super::generate(&EXAMPLE_INPUT.lines().skip(5).join("\n")),
            get_example_data()
        );
    }

    #[test]
    fn day7_part1() {
        assert_eq!(super::day7_part1(&super::generate(EXAMPLE_INPUT)), 4);
    }

    #[test]
    fn day7_part2() {
        assert_eq!(super::day7_part2(&super::generate(EXAMPLE_INPUT)), 32);
        assert_eq!(super::day7_part2(&super::generate(EXAMPLE_INPUT2)), 126);
    }
}
