use aoc_runner_derive::{aoc, aoc_generator};
use nom::{AsChar, IResult};
use std::collections::HashMap;

type Passport = HashMap<String, String>;
type GeneratorOutput = Vec<Passport>;
type PartInput = [Passport];

#[aoc_generator(day4)]
pub fn generator(input: &str) -> anyhow::Result<GeneratorOutput> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        combinator::{all_consuming, map},
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
        Some(birth_year) if (1920..=2002).contains(&birth_year) => {}
        _ => return false,
    }
    match passport.get("iyr").and_then(|iyr| iyr.parse::<u32>().ok()) {
        Some(issue_year) if (2010..=2020).contains(&issue_year) => {}
        _ => return false,
    }
    match passport.get("eyr").and_then(|eyr| eyr.parse::<u32>().ok()) {
        Some(expiration_year) if (2020..=2030).contains(&expiration_year) => {}
        _ => return false,
    }
    match passport
        .get("hgt")
        .and_then(|hgt| parse_height(hgt).ok())
        .map(|(_, v)| v)
    {
        Some((height, HeightUnit::Centimeters)) if (150..=193).contains(&height) => {}
        Some((height, HeightUnit::Inches)) if (59..=76).contains(&height) => {}
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
    true
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum HeightUnit {
    Centimeters,
    Inches,
}

fn parse_height(input: &str) -> IResult<&str, (u32, HeightUnit)> {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while},
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
    use nom::{
        bytes::complete::{tag, take_while_m_n},
        combinator::map,
    };

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(
            part_1(
                &generator(
                    "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
                )
                .unwrap()
            ),
            2
        );
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            part_2(
                &generator(
                    "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
                )
                .unwrap()
            ),
            0
        );
        assert_eq!(
            part_1(
                &generator(
                    "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
                )
                .unwrap()
            ),
            4
        );
    }
}
