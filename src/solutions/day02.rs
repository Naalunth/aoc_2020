use aoc_runner_derive::{aoc, aoc_generator};
use nom::IResult;

type GeneratorOutput = Vec<PasswordEntry>;
type PartInput = [PasswordEntry];

#[aoc_generator(day2)]
pub fn generator(input: &[u8]) -> anyhow::Result<GeneratorOutput> {
    use nom::{bytes::complete::tag, combinator::all_consuming, multi::separated_list0};
    Ok(
        all_consuming(separated_list0(tag(b"\n"), parse_password))(input)
            .map_err(|e| e.map(|e| (e.input, e.code)).to_owned())?
            .1,
    )
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PasswordEntry {
    min: u8,
    max: u8,
    letter: u8,
    password: Vec<u8>,
}

fn parse_password(input: &[u8]) -> IResult<&[u8], PasswordEntry> {
    use crate::util::parsers::unsigned_number;
    use nom::bytes::complete::{tag, take, take_till};
    let (input, min) = unsigned_number::<u8>(input)?;
    let (input, _) = tag("-")(input)?;
    let (input, max) = unsigned_number::<u8>(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, letter) = take(1usize)(input)?;
    let letter = letter[0];
    let (input, _) = tag(": ")(input)?;
    let (input, password) = take_till(|c| c == b'\n')(input)?;

    Ok((
        input,
        PasswordEntry {
            min,
            max,
            letter,
            password: password.to_owned(),
        },
    ))
}

#[aoc(day2, part1)]
pub fn part_1(input: &PartInput) -> usize {
    input.iter().filter(|&pw| pw.is_valid()).count()
}

impl PasswordEntry {
    fn is_valid(&self) -> bool {
        let count = bytecount::count(&self.password, self.letter) as u8;
        count >= self.min && count <= self.max
    }

    fn is_valid_2(&self) -> bool {
        [self.min, self.max]
            .iter()
            .filter(|&idx| self.password[(idx - 1) as usize] == self.letter)
            .count()
            == 1
    }
}

#[aoc(day2, part2)]
pub fn part_2(input: &PartInput) -> usize {
    input.iter().filter(|&pw| pw.is_valid_2()).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const RULES: &'static [u8] = b"1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(&generator(RULES).unwrap()), 2);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(&generator(RULES).unwrap()), 1);
    }
}
