use std::collections::HashMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

static REQUIRED_FIELDS: [&str; 7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid"
];

lazy_static! {
    static ref HEX_COLOR_PATTERN: Regex = Regex::new(r##"#[a-f\d]{6}"##).unwrap();
}

static VALID_EYE_COLORS: [&str; 7] = [
    "amb",
    "blu",
    "brn",
    "gry",
    "grn",
    "hzl",
    "oth"
];

fn parse_record(input: &str) -> HashMap<&str, &str> {
    input
        .split_whitespace()
        .flat_map(|fv| fv.split(':'))
        .tuples()
        .collect()
}

fn has_required_fields(record: &HashMap<&str, &str>) -> bool {
    REQUIRED_FIELDS.iter().all(|k| record.contains_key(k))
}

pub fn day4_part1(input: String) -> i64 {
    input.split("\n\n")
        .map(parse_record)
        .filter(has_required_fields)
        .count() as i64
}

fn field_is_valid((field, value): (&&str, &&str)) -> bool {
    match *field {
        "byr" => value.parse::<u32>()
            .map(|x| x >= 1920 && x <= 2002)
            .unwrap_or(false),
        "iyr" => value.parse::<u32>()
            .map(|x| x >= 2010 && x <= 2020)
            .unwrap_or(false),
        "eyr" => value.parse::<u32>()
            .map(|x| x >= 2020 && x <= 2030)
            .unwrap_or(false),
        "hgt" => {
            let unit = &value[value.len() - 2..value.len()];
            let height = (&value[0..value.len() - 2]).parse::<u32>();
            height.map(|h| match unit {
                "in" => h >= 59 && h <= 76,
                "cm" => h >= 150 && h <= 193,
                _ => false
            }).unwrap_or(false)
        },
        "hcl" => HEX_COLOR_PATTERN.is_match(value),
        "ecl" => VALID_EYE_COLORS.contains(&value),
        "pid" => value.len() == 9 && value.chars().all(char::is_numeric),
        _ => true
    }
}

fn is_valid(record: &HashMap<&str, &str>) -> bool {
    has_required_fields(record) && record.iter().all(field_is_valid)
}

pub fn day4_part2(input: String) -> i64 {
    input.split("\n\n")
        .map(parse_record)
        .filter(is_valid)
        .count() as i64
}