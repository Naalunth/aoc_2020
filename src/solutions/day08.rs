use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use std::mem::replace;

type GeneratorOutput = Vec<Instruction>;
type PartInput = [Instruction];

#[aoc_generator(day8)]
pub fn generator(input: &str) -> anyhow::Result<GeneratorOutput> {
    input
        .split('\n')
        .map(|line| {
            let op = match &line[..3] {
                "nop" => Operation::Nop,
                "jmp" => Operation::Jmp,
                "acc" => Operation::Acc,
                _ => anyhow::bail!("unknown opcode"),
            };
            let arg = line[4..].parse::<i64>().context("number parsing failed")?;
            Ok(Instruction { op, arg })
        })
        .collect()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction {
    op: Operation,
    arg: i64,
}

#[aoc(day8, part1)]
pub fn part_1(input: &PartInput) -> i64 {
    if let RunResult::InfiniteLoop(result) = run_code(input) {
        return result;
    }
    unreachable!()
}

#[aoc(day8, part2)]
pub fn part_2(input: &PartInput) -> i64 {
    fn swap_op(op: &mut Operation) {
        match op {
            op @ Operation::Jmp => *op = Operation::Nop,
            op @ Operation::Nop => *op = Operation::Jmp,
            _ => unreachable!(),
        }
    }

    let mut code = input.to_vec();
    for idx in 0..code.len() {
        if code[idx].op != Operation::Acc {
            swap_op(&mut code[idx].op);
            match run_code(&code) {
                RunResult::InfiniteLoop(_) | RunResult::Error => {
                    swap_op(&mut code[idx].op);
                }
                RunResult::Terminated(result) => {
                    return result;
                }
            }
        }
    }
    unreachable!()
}

enum RunResult {
    InfiniteLoop(i64),
    Terminated(i64),
    Error,
}

fn run_code(input: &[Instruction]) -> RunResult {
    let mut has_visited_line_before = vec![false; input.len()];
    let mut pc = 0usize;
    let mut acc = 0i64;

    loop {
        if pc == input.len() {
            return RunResult::Terminated(acc);
        }
        if pc > input.len() {
            return RunResult::Error;
        }
        if replace(&mut has_visited_line_before[pc], true) {
            return RunResult::InfiniteLoop(acc);
        }
        let instruction = input[pc];
        match instruction.op {
            Operation::Acc => {
                acc += instruction.arg;
                pc += 1;
            }
            Operation::Jmp => {
                pc += instruction.arg as usize;
            }
            Operation::Nop => {
                pc += 1;
            }
        }
    }
}
