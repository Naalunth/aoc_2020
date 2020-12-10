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

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE1: &str = "16
10
15
5
1
11
7
19
6
12
4";
    const EXAMPLE2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(&generator(EXAMPLE1).unwrap()), 35);
        assert_eq!(part_1(&generator(EXAMPLE2).unwrap()), 220);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(&generator(EXAMPLE1).unwrap()), 8);
        assert_eq!(part_2(&generator(EXAMPLE2).unwrap()), 19208);
    }
}
