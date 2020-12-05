use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{char, digit1, hex_digit1},
    sequence::tuple,
};
use std::collections::{HashMap, HashSet};

fn valid_hgt(input: &str) -> bool {
    let result: nom::IResult<_, _> = tuple((digit1, alt((tag("cm"), tag("in")))))(input);
    match result {
        Ok((_, (num, "in"))) => (59..=76).contains(&num.parse::<u32>().unwrap_or(0)),
        Ok((_, (num, "cm"))) => (150..=193).contains(&num.parse::<u32>().unwrap_or(0)),
        _ => false,
    }
}

fn valid_ecl(input: &str) -> bool {
    let result: nom::IResult<_, _> = alt((
        tag("amb"),
        tag("blu"),
        tag("brn"),
        tag("gry"),
        tag("grn"),
        tag("hzl"),
        tag("oth"),
    ))(input);
    result.map(|(remaining, _)| remaining) == Ok("")
}

fn valid_pid(input: &str) -> bool {
    let result: nom::IResult<_, _> = digit1(input);
    result.map(|(_, num)| num.len()) == Ok(9)
}

fn valid_hcl(input: &str) -> bool {
    let result: nom::IResult<_, _> = tuple((char('#'), hex_digit1))(input);
    result.map(|(_, (_, hex))| hex.len()) == Ok(6)
}

fn valid_passport(m: &HashMap<String, String>) -> Option<bool> {
    Some(
        (1920..=2002).contains(&m.get("byr")?.parse::<u32>().ok()?)
            && (2010..=2020).contains(&m.get("iyr")?.parse::<u32>().ok()?)
            && (2020..=2030).contains(&m.get("eyr")?.parse::<u32>().ok()?)
            && valid_hgt(&m.get("hgt")?)
            && valid_hcl(&m.get("hcl")?)
            && valid_ecl(&m.get("ecl")?)
            && valid_pid(&m.get("pid")?),
    )
}

fn valid_passport_part1(m: &HashMap<String, String>) -> bool {
    let required_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    m.keys()
        .map(|s| s.to_owned())
        .collect::<HashSet<_>>()
        .is_superset(&required_keys.into_iter().map(|s| s.to_owned()).collect())
}

fn main() {
    let input = std::fs::read_to_string("input_day4.txt").unwrap();

    let passports = input
        .lines()
        .batching(|i| {
            i.skip_while(|s| s.is_empty())
                .take_while(|s| !s.is_empty())
                .fold(None, |acc, s| {
                    Some(format!("{} {}", acc.unwrap_or_else(String::new), s))
                })
        })
        .map(|passport| {
            passport
                .split(" ")
                .filter_map(|s| {
                    s.split(':')
                        .next_tuple()
                        .map(|(a, b)| (a.to_owned(), b.to_owned()))
                })
                .collect::<HashMap<_, _>>()
        });

    let part2_passports = passports.clone();

    println!(
        "Part 1: {}, part 2: {}",
        passports.filter(|p| valid_passport_part1(p)).count(),
        part2_passports
            .filter(|p| valid_passport(p) == Some(true))
            .count()
    );
}
