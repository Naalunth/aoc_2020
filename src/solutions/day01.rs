use aoc_runner_derive::{aoc, aoc_generator};
use fnv::FnvHashSet;

type GeneratorOutput = Vec<u32>;
type PartInput = [u32];

#[aoc_generator(day1)]
pub fn generator(input: &[u8]) -> anyhow::Result<GeneratorOutput> {
    use crate::util::parsers::unsigned_number;
    use nom::{bytes::complete::tag, combinator::all_consuming, multi::separated_list0};
    Ok(
        all_consuming(separated_list0(tag(b"\n"), unsigned_number::<u32>))(input)
            .map_err(|e| e.map(|e| (e.input, e.code)).to_owned())?
            .1,
    )
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
    for x in input {
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
    for (idx, a) in input.iter().enumerate() {
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
