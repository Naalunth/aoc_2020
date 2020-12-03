use aoc_runner_derive::{aoc, aoc_generator};
use bstr::ByteSlice;
use ndarray::Array2;
use std::mem::MaybeUninit;

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
    let row_count = bytecount::count(input, b'\n') + 1;
    let column_count = input.find_byte(b'\n').unwrap();
    let mut result = Array2::maybe_uninit((row_count, column_count));
    result
        .iter_mut()
        .zip(input.iter().cloned().filter_map(|c| match c {
            b'.' => Some(MapElement::Empty),
            b'#' => Some(MapElement::Tree),
            _ => None,
        }))
        .for_each(|(array_spot, elem)| *array_spot = MaybeUninit::new(elem));

    Ok(unsafe { result.assume_init() })
}

#[aoc(day3, part1)]
pub fn part_1(input: &PartInput) -> usize {
    count_trees(input, 1, 3)
}

fn count_trees(map: &Array2<MapElement>, down: usize, right: usize) -> usize {
    let (height, width) = if let [h, w] = *map.shape() {
        (h, w)
    } else {
        unreachable!()
    };

    let mut x = 0usize;
    let mut y = 0usize;
    let mut count = 0usize;

    while y < height {
        count += map[(y, x)] as usize;
        y += down;
        x = (x + right) % width;
    }

    count
}

#[aoc(day3, part2)]
pub fn part_2(input: &PartInput) -> usize {
    [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .iter()
        .map(|&(down, right)| count_trees(input, down, right))
        .product()
}
