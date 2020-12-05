use anyhow::Context;
use aoc_runner_derive::{aoc, aoc_generator};
use memchr::memchr;
use ndarray::{Array, Array2};
use std::iter::successors;

type GeneratorOutput = Array2<MapElement>;
type PartInput = GeneratorOutput;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum MapElement {
    Empty = 0,
    Tree = 1,
}

#[aoc_generator(day3)]
pub fn generator(input: &[u8]) -> anyhow::Result<GeneratorOutput> {
    let height = bytecount::count(input, b'\n') + 1;
    let width = memchr(b'\n', input).unwrap();
    input
        .iter()
        .filter_map(|&c| match c {
            b'.' => Some(MapElement::Empty),
            b'#' => Some(MapElement::Tree),
            _ => None,
        })
        .collect::<Array<_, _>>()
        .into_shape([height, width])
        .context("reshape failed")
}

#[aoc(day3, part1)]
pub fn part_1(input: &PartInput) -> usize {
    count_trees(input, 1, 3)
}

pub fn count_trees(map: &Array2<MapElement>, down: usize, right: usize) -> usize {
    let height = map.nrows();
    let width = map.ncols();

    successors(Some((0, 0)), |&(y, x)| match y + down {
        y if y < height => Some((y, (x + right) % width)),
        _ => None,
    })
    .map(|c| map[c] as usize)
    .sum()
}

#[aoc(day3, part2)]
pub fn part_2(input: &PartInput) -> u128 {
    [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|&(down, right)| count_trees(input, down, right) as u128)
        .product()
}
