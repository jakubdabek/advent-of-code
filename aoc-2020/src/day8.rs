use aoc_runner_derive::{aoc, aoc_generator};
use aoc_utils::try_from_lines;
use std::convert::TryFrom;

type Ip = i32;
type Acc = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    Acc(Acc),
    Jmp(Ip),
    Nop(Ip),
}

impl Op {
    fn invert(&self) -> Op {
        use Op::*;
        match self {
            &Jmp(v) => Nop(v),
            &Nop(v) => Jmp(v),
            op => op.clone(),
        }
    }
}

impl TryFrom<&str> for Op {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let value = s[4..].parse().map_err(|_| ())?;

        Ok(match &s[..3] {
            "acc" => Op::Acc(value),
            "jmp" => Op::Jmp(value),
            "nop" => Op::Nop(value),
            _ => return Err(()),
        })
    }
}

#[aoc_generator(day8)]
pub fn generate(s: &str) -> Vec<Op> {
    try_from_lines(s).expect("couldn't parse input")
}

type ExecResult = Result<Acc, Acc>;
trait OnVisited: FnOnce(&[Op], Ip, Acc, Vec<bool>, Vec<Ip>) -> Option<ExecResult> {}
impl<F: FnOnce(&[Op], Ip, Acc, Vec<bool>, Vec<Ip>) -> Option<ExecResult>> OnVisited for F {}

fn on_visited_nop(_: &[Op], _: Ip, _: Acc, _: Vec<bool>, _: Vec<Ip>) -> Option<ExecResult> {
    None
}

fn execute(instructions: &[Op], on_visited: impl OnVisited, inverted: Option<Ip>) -> ExecResult {
    let mut visited = vec![false; instructions.len()];
    let mut visit_order = Vec::with_capacity(instructions.len());

    let mut ip = 0i32;
    let mut acc = 0;

    while ip < instructions.len() as i32 {
        if std::mem::replace(&mut visited[ip as usize], true) {
            return if let Some(value) = on_visited(instructions, ip, acc, visited, visit_order) {
                value
            } else {
                Err(acc)
            };
        }
        visit_order.push(ip);

        let instr = instructions[ip as usize];
        let instr = if inverted == Some(ip) {
            instr.invert()
        } else {
            instr
        };
        match instr {
            Op::Acc(v) => acc += v,
            Op::Jmp(offset) => {
                ip += offset;
                continue;
            }
            Op::Nop(_) => {}
        }
        ip += 1
    }

    Ok(acc)
}

#[aoc(day8, part1)]
pub fn day8_part1(instructions: &[Op]) -> i32 {
    execute(instructions, on_visited_nop, None).unwrap_err()
}

#[aoc(day8, part2)]
pub fn day8_part2(instructions: &[Op]) -> i32 {
    fn on_visited(
        instructions: &[Op],
        _: Ip,
        _: Acc,
        _: Vec<bool>,
        visit_order: Vec<Ip>,
    ) -> Option<ExecResult> {
        for visited in visit_order {
            if let Op::Jmp(_) | Op::Nop(_) = instructions[visited as usize] {
                if let result @ Ok(_) = execute(instructions, on_visited_nop, Some(visited)) {
                    return Some(result);
                }
            }
        }

        panic!("couldn't find solution")
    }
    execute(instructions, on_visited, None).unwrap()
}

#[cfg(test)]
mod tests {
    use super::Op;

    const EXAMPLE_INPUT: &str = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    fn get_example_data() -> Vec<Op> {
        use super::Op::*;
        vec![
            Nop(0),
            Acc(1),
            Jmp(4),
            Acc(3),
            Jmp(-3),
            Acc(-99),
            Acc(1),
            Jmp(-4),
            Acc(6),
        ]
    }

    #[test]
    fn generate() {
        assert_eq!(super::generate(EXAMPLE_INPUT), get_example_data());
    }

    #[test]
    fn day8_part1() {
        assert_eq!(super::day8_part1(&get_example_data()), 5);
    }

    #[test]
    fn day8_part2() {
        assert_eq!(super::day8_part2(&get_example_data()), 8);
    }
}
