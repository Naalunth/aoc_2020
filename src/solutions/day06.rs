use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::HashSet;

#[aoc(day6, part1)]
pub fn part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|&c| c != '\n')
                .collect::<HashSet<_>>()
                .iter()
                .count()
        })
        .sum()
}

#[aoc(day6, part2)]
pub fn part_2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|member| member.chars().collect::<HashSet<_>>())
                .fold1(|a, b| a.intersection(&b).cloned().collect())
                .unwrap()
                .iter()
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const LIST: &str = "abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(LIST), 11);
    }
    #[test]
    fn part_2_test() {
        assert_eq!(part_2(LIST), 6);
    }
}
