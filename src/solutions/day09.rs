use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Ordering;

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
    part_1_inner(input, 25)
}

fn part_1_inner(input: &[u64], preamble_length: usize) -> u64 {
    for (idx, &number) in input.iter().enumerate().skip(preamble_length) {
        if !is_sum_of_two(number, &input[(idx - preamble_length)..idx]) {
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
                return min_max_sum(&input[base_idx..=end_idx]);
            }
        }
    }
    unreachable!()
}

#[aoc(day9, part2, sliding_window)]
pub fn part_2_sliding_window(input: &PartInput) -> u64 {
    let target = part_1(input);
    let mut window_size = 0usize;
    let mut previous_ordering = Ordering::Less;
    let mut base_idx = 0;
    let mut sum = 0u64;
    loop {
        match previous_ordering {
            Ordering::Equal => break,
            Ordering::Less => {
                sum += input[base_idx + window_size];
                window_size += 1;
            }
            Ordering::Greater => {
                sum -= input[base_idx + window_size - 1];
                window_size -= 1;
            }
        }
        match sum.cmp(&target) {
            Ordering::Equal => break,
            new_ordering if new_ordering == previous_ordering => {}
            _ => {
                sum -= input[base_idx];
                sum += input[base_idx + window_size];
                base_idx += 1;
                previous_ordering = sum.cmp(&target);
            }
        }
    }
    min_max_sum(&input[base_idx..(base_idx + window_size)])
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

fn min_max_sum(range: &[u64]) -> u64 {
    let (min, max) = range.iter().minmax().into_option().unwrap();
    min + max
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn part_1_test() {
        assert_eq!(part_1_inner(&generator(EXAMPLE).unwrap(), 5), 127);
    }
}
