use aoc_runner_derive::{aoc, aoc_generator};
use nom::{AsChar, IResult};
use std::collections::HashMap;

type Passport = HashMap<String, String>;
type GeneratorOutput = Vec<Passport>;
type PartInput = [Passport];

#[aoc_generator(day4)]
pub fn generator(input: &str) -> anyhow::Result<GeneratorOutput> {
    use nom::{
        branch::alt, bytes::complete::tag, combinator::all_consuming, combinator::map,
        multi::separated_list1,
    };
    Ok(all_consuming(separated_list1(
        tag("\n\n"),
        map(
            separated_list1(alt((tag(" "), tag("\n"))), parse_field),
            |list| list.into_iter().collect(),
        ),
    ))(input)
    .map_err(|e| e.map(|e| (e.input, e.code)).to_owned())?
    .1)
}

fn parse_field(input: &str) -> IResult<&str, (String, String)> {
    use nom::bytes::complete::{tag, take_while};
    let (input, key) = take_while(|c: char| c.is_ascii_alphabetic())(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, value) = take_while(|c: char| !c.is_ascii_whitespace())(input)?;

    Ok((input, (key.to_owned(), value.to_owned())))
}

#[aoc(day4, part1)]
pub fn part_1(input: &PartInput) -> usize {
    const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    input
        .iter()
        .filter(|&passport| REQUIRED_KEYS.iter().all(|&key| passport.contains_key(key)))
        .count()
}

#[aoc(day4, part2)]
pub fn part_2(input: &PartInput) -> usize {
    input
        .iter()
        .filter(|&passport| is_passport_valid(passport))
        .count()
}

fn is_passport_valid(passport: &Passport) -> bool {
    match passport.get("byr").and_then(|byr| byr.parse::<u32>().ok()) {
        Some(birth_year) if birth_year >= 1920 && birth_year <= 2002 => {}
        _ => return false,
    }
    match passport.get("iyr").and_then(|iyr| iyr.parse::<u32>().ok()) {
        Some(issue_year) if issue_year >= 2010 && issue_year <= 2020 => {}
        _ => return false,
    }
    match passport.get("eyr").and_then(|eyr| eyr.parse::<u32>().ok()) {
        Some(expiration_year) if expiration_year >= 2020 && expiration_year <= 2030 => {}
        _ => return false,
    }
    match passport
        .get("hgt")
        .and_then(|hgt| parse_height(hgt).ok())
        .map(|(_, v)| v)
    {
        Some((height, HeightUnit::Centimeters)) if height >= 150 && height <= 193 => {}
        Some((height, HeightUnit::Inches)) if height >= 59 && height <= 76 => {}
        _ => return false,
    }
    match passport
        .get("hcl")
        .and_then(|hcl| parse_color_code(hcl).ok())
    {
        Some(_) => {}
        _ => return false,
    }
    const VALID_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    match passport.get("ecl") {
        Some(eye_color) if VALID_COLORS.contains(&eye_color.as_str()) => {}
        _ => return false,
    }
    match passport.get("pid") {
        Some(pid) if pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit()) => {}
        _ => return false,
    }
    return true;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum HeightUnit {
    Centimeters,
    Inches,
}

fn parse_height(input: &str) -> IResult<&str, (u32, HeightUnit)> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        bytes::complete::take_while,
        combinator::{map, value},
    };
    let (input, number) = map(take_while(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<u32>().unwrap()
    })(input)?;
    let (input, unit) = alt((
        value(HeightUnit::Centimeters, tag("cm")),
        value(HeightUnit::Inches, tag("in")),
    ))(input)?;

    Ok((input, (number, unit)))
}

fn parse_color_code(input: &str) -> IResult<&str, (u8, u8, u8)> {
    use nom::{bytes::complete::tag, bytes::complete::take_while_m_n, combinator::map};

    let (input, _) = tag("#")(input)?;
    let mut byte_parser = map(
        take_while_m_n(2, 2, |c: char| c.is_hex_digit()),
        |s: &str| u8::from_str_radix(s, 16).unwrap(),
    );
    let (input, r) = byte_parser(input)?;
    let (input, g) = byte_parser(input)?;
    let (input, b) = byte_parser(input)?;

    Ok((input, (r, g, b)))
}
