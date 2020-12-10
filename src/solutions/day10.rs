use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use std::iter::once;

type GeneratorOutput = Vec<u64>;
type PartInput = [u64];

#[aoc_generator(day10)]
pub fn generator(input: &str) -> anyhow::Result<GeneratorOutput> {
    input
        .lines()
        .map(|line| line.parse().context("number parsing failed"))
        .collect()
}

#[aoc(day10, part1)]
pub fn part_1(input: &PartInput) -> u64 {
    let mut adapters = input.to_vec();
    adapters.sort_unstable();
    let mut count_1 = 0;
    let mut count_3 = 0;
    let mut last = 0;

    for joltage in adapters {
        match joltage - last {
            1 => count_1 += 1,
            3 => count_3 += 1,
            _ => unreachable!(),
        }

        last = joltage;
    }
    count_1 * (count_3 + 1)
}

#[aoc(day10, part2)]
pub fn part_2(input: &PartInput) -> u64 {
    let mut adapters = input.to_vec();
    adapters.sort_unstable();
    let laptop = adapters.last().unwrap() + 3;
    let mut last = 0;
    let mut current_run = 0u64;

    let mut memo = Vec::new();
    let mut combinations = 1;

    for joltage in adapters.into_iter().chain(once(laptop)) {
        match joltage - last {
            1 => current_run += 1,
            3 => {
                combinations *= arrangements(current_run, &mut memo);
                current_run = 0
            }
            _ => unreachable!(),
        }

        last = joltage;
    }

    combinations
}

fn arrangements(run_length: u64, memo: &mut Vec<u64>) -> u64 {
    if let Some(result) = memo.get(run_length as usize) {
        return *result;
    }
    let result = match run_length {
        0 | 1 => 1,
        2 => 2,
        _ => {
            arrangements(run_length - 3, memo)
                + arrangements(run_length - 2, memo)
                + arrangements(run_length - 1, memo)
        }
    };

    memo.resize((run_length + 1) as usize, 1);
    memo[run_length as usize] = result;
    result
}
