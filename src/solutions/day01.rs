use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use fnv::FnvHashSet;

type GeneratorOutput = Vec<u32>;
type PartInput = [u32];

#[aoc_generator(day1)]
pub fn generator(input: &[u8]) -> anyhow::Result<GeneratorOutput> {
    input
        .split(|b| *b == b'\n')
        .map(|string| btoi::btou(string))
        .collect::<Result<_, _>>()
        .context("parser error")
}

#[aoc(day1, part1, naive)]
pub fn part_1(input: &PartInput) -> u32 {
    for (idx, a) in input.iter().enumerate() {
        for b in &input[idx + 1..] {
            if a + b == 2020 {
                return a * b;
            }
        }
    }
    unreachable!()
}

#[aoc(day1, part2, naive)]
pub fn part_2(input: &PartInput) -> u32 {
    for (idx_a, a) in input.iter().enumerate() {
        for (idx_b, b) in input[idx_a + 1..].iter().enumerate() {
            for c in &input[idx_b + 1..] {
                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }
    unreachable!()
}

#[aoc(day1, part1, single_pass)]
pub fn part_1_single(input: &PartInput) -> u32 {
    let mut set = FnvHashSet::with_capacity_and_hasher(input.len(), Default::default());
    for x in input {
        let comp = 2020 - x;
        if set.contains(&comp) {
            return x * comp;
        }
        set.insert(*x);
    }
    unreachable!()
}

#[aoc(day1, part1, single_pass_array)]
pub fn part_1_single_array(input: &PartInput) -> u32 {
    let mut flags = [false; 2020];
    flags[input[0] as usize] = true;
    for x in &input[1..] {
        let comp = 2020 - x;
        if flags[comp as usize] {
            return x * comp;
        }
        flags[*x as usize] = true;
    }
    unreachable!()
}

#[aoc(day1, part2, one_pass_less)]
pub fn part_2_one_pass_less(input: &PartInput) -> u32 {
    let mut flags = [false; 2020];
    flags[input[0] as usize] = true;
    for (idx, a) in input[1..].iter().enumerate() {
        for b in &input[idx + 1..] {
            if a + b <= 2020 {
                let comp = 2020 - (a + b);
                if flags[comp as usize] {
                    return a * b * comp;
                }
            }
        }
        flags[*a as usize] = true;
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const LIST: [u32; 6] = [1721, 979, 366, 299, 675, 1456];

    fn part_1_tests(f: fn(input: &PartInput) -> u32) {
        assert_eq!(f(&LIST), 514579);
    }

    #[test]
    fn part_1() {
        part_1_tests(super::part_1);
    }

    #[test]
    fn part_1_single() {
        part_1_tests(super::part_1_single);
    }

    #[test]
    fn part_1_single_array() {
        part_1_tests(super::part_1_single_array);
    }

    fn part_2_tests(f: fn(input: &PartInput) -> u32) {
        assert_eq!(f(&LIST), 241861950);
    }

    #[test]
    fn part_2() {
        part_2_tests(super::part_2);
    }

    #[test]
    fn part_2_one_pass_less() {
        part_2_tests(super::part_2_one_pass_less);
    }
}
