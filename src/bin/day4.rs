use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::streaming::tag,
    character::complete::{char, digit1, hex_digit1},
    sequence::tuple,
    IResult,
};
use std::collections::HashMap;

fn part1() {
    let mut passport_lines = Vec::new();
    let mut s = String::new();
    for line in std::fs::read_to_string("input_day4.txt").unwrap().lines() {
        if line == "" {
            passport_lines.push(s.clone());
            s.clear();
        } else {
            s.push_str(line);
            s.push_str(" ");
        }
    }
    println!("{:#?}", passport_lines);
    println!("{:#?}", passport_lines.len());
    let valid = passport_lines
        .iter()
        .filter(|s| s.contains("byr:"))
        .filter(|s| s.contains("iyr:"))
        .filter(|s| s.contains("eyr:"))
        .filter(|s| s.contains("hgt:"))
        .filter(|s| s.contains("hcl:"))
        .filter(|s| s.contains("ecl:"))
        .filter(|s| s.contains("pid:"))
        .count();
    println!("{}", valid);
}

fn get_next_passport<'a, I>(i: &mut I) -> String
where
    I: Iterator<Item = &'a str>,
{
    i.skip_while(|s| s.is_empty())
        .take_while(|s| !s.is_empty())
        .fold(String::new(), |acc, s| format!("{} {}", acc, s))
}

fn valid_hgt(input: &str) -> bool {
    let result: IResult<_, _> = tuple((digit1, alt((tag("cm"), tag("in")))))(input);
    if let Ok((_, (num, type_))) = result {
        if type_ == "in" {
            (59..=76).contains(&num.parse::<u32>().unwrap_or(0))
        } else {
            (150..=193).contains(&num.parse::<u32>().unwrap_or(0))
        }
    } else {
        false
    }
}

fn valid_ecl(input: &str) -> bool {
    let result: IResult<_, _> = alt((
        tag("amb"),
        tag("blu"),
        tag("brn"),
        tag("gry"),
        tag("grn"),
        tag("hzl"),
        tag("oth"),
    ))(input);
    if let Ok((remaining, _)) = result {
        remaining == ""
    } else {
        false
    }
}

fn valid_pid(input: &str) -> bool {
    let result: IResult<_, _> = digit1(input);
    if let Ok((_, num)) = result {
        num.len() == 9
    } else {
        false
    }
}

fn valid_hcl(input: &str) -> bool {
    let result: IResult<_, _> = tuple((char('#'), hex_digit1))(input);
    if let Ok((_, (_, hex))) = result {
        hex.len() == 6
    } else {
        false
    }
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

fn part2() {
    let input = std::fs::read_to_string("input_day4.txt").unwrap();
    let raw_lines = input.lines().peekable();

    let passports: Vec<HashMap<_, _>> = raw_lines
        .batching(|i| {
            if i.peek().is_some() {
                Some(get_next_passport(i))
            } else {
                None
            }
        })
        .map(|passport| {
            passport
                .split(" ")
                .filter_map(|s| {
                    if s.is_empty() {
                        None
                    } else {
                        let mut parts = s.split(':');
                        Some((
                            parts.next().unwrap().to_owned(),
                            parts.next().unwrap().to_owned(),
                        ))
                    }
                })
                .collect()
        })
        .collect();

    println!("{:#?}", passports);
    println!(
        "{}",
        passports
            .iter()
            .filter(|p| valid_passport(p) == Some(true))
            .count()
    );
}

fn main() {
    part1();
    part2();
}
