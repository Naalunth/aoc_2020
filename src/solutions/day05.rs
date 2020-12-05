use aoc_runner_derive::aoc;
use std::hint::unreachable_unchecked;

#[aoc(day5, part1)]
pub fn part_1(input: &[u8]) -> u16 {
    parse_input(input)
        .max()
        .unwrap_or_else(|| unsafe { unreachable_unchecked() })
}

fn parse_input(input: &[u8]) -> impl Iterator<Item = u16> + '_ {
    input
        .chunks(11)
        .map(|chunk| id(unsafe { chunk.get_unchecked(..10) }))
}

fn id(seat: &[u8]) -> u16 {
    unsafe {
        debug_assert!(seat.len() == 10);
        if seat.len() != 10 {
            unreachable_unchecked()
        }
        let translate = |idx, letter| ((*seat.get_unchecked(idx) == letter) as u16) << (9 - idx);
        (0..=6).fold(0u16, |acc, idx| acc | translate(idx, b'B'))
            | (7..=9).fold(0u16, |acc, idx| acc | translate(idx, b'R'))
    }
}

#[aoc(day5, part2)]
pub fn part_2(input: &[u8]) -> u32 {
    let mut min = 1024;
    let mut max = 0;
    let mut sum = 0;

    for id in parse_input(input) {
        min = min.min(id);
        max = max.max(id);
        sum += id as u32;
    }

    let min = min as u32;
    let max = max as u32;

    let target = max * (max + 1) / 2 - (min - 1) * min / 2;
    target - sum
}
