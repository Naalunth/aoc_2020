use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, MinMaxResult};

type GeneratorOutput = Vec<u64>;
type PartInput = [u64];

#[aoc_generator(day9)]
pub fn generator(input: &str) -> anyhow::Result<GeneratorOutput> {
    input
        .split('\n')
        .map(|line| line.parse().context("number parsing failed"))
        .collect()
}

#[aoc(day9, part1)]
pub fn part_1(input: &PartInput) -> u64 {
    for (idx, &number) in input.iter().enumerate().skip(25) {
        if !is_sum_of_two(number, &input[(idx - 25)..idx]) {
            return number;
        }
    }
    unreachable!()
}

#[aoc(day9, part2)]
pub fn part_2(input: &PartInput) -> u64 {
    let target = part_1(input);
    for base_idx in 0..input.len() {
        if let Some((end_idx, sum)) = input
            .iter()
            .enumerate()
            .skip(base_idx)
            .scan(0u64, |state, (idx, &x)| {
                *state += x;
                if *state <= target {
                    Some((idx, *state))
                } else {
                    None
                }
            })
            .last()
        {
            if sum == target {
                match input[base_idx..=end_idx].iter().minmax() {
                    MinMaxResult::NoElements => {
                        unreachable!()
                    }
                    MinMaxResult::OneElement(x) => {
                        return x + x;
                    }
                    MinMaxResult::MinMax(min, max) => {
                        return min + max;
                    }
                }
            }
        }
    }
    unreachable!()
}

fn is_sum_of_two(number: u64, others: &[u64]) -> bool {
    for (idx, a) in others.iter().enumerate() {
        for b in &others[idx + 1..] {
            if a + b == number {
                return true;
            }
        }
    }
    false
}
