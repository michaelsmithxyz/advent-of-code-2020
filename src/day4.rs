use std::vec::Vec;
use std::collections::HashMap;
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

type Validator = fn(&str) -> bool;

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

lazy_static! {
    static ref VALIDATORS: HashMap<&'static str, Validator> = {
        let mut validators: HashMap<&'static str, Validator> = HashMap::new();

        validators.insert("byr", |s| {
            s.len() == 4 && {
                let v = s.parse::<u32>();
                v.map(|x| x >= 1920 && x <= 2002)
                    .unwrap_or(false)
            }
        });

        validators.insert("iyr", |s| {
            s.len() == 4 && {
                let v = s.parse::<u32>();
                v.map(|x| x >= 2010 && x <= 2020)
                    .unwrap_or(false)
            }
        });

        validators.insert("eyr", |s| {
            s.len() == 4 && {
                let v = s.parse::<u32>();
                v.map(|x| x >= 2020 && x <= 2030)
                    .unwrap_or(false)
            }
        });

        validators.insert("hgt", |s| {
            s.ends_with("in") && {
                s.replace("in", "")
                    .parse::<u32>()
                    .map(|i| i >= 59 && i <= 76)
                    .unwrap_or(false)
            } || s.ends_with("cm") && {
                s.replace("cm", "")
                    .parse::<u32>()
                    .map(|i| i >= 150 && i <= 193)
                    .unwrap_or(false)
            }
        });

        validators.insert("hcl", |s| {
            HEX_COLOR_PATTERN.is_match(s)
        });

        validators.insert("ecl", |s| {
            VALID_EYE_COLORS.contains(&s)
        });

        validators.insert("pid", |s| {
            s.len() == 9 && s.chars().all(char::is_numeric)
        });

        validators.insert("cid", |_| true);

        validators
    };
}

fn parse_record(input: &str) -> HashMap<&str, &str> {
    input
        .split_whitespace()
        .map(|fv| {
            let parts: Vec<&str> = fv.split(':').collect();
            (parts[0], parts[1])
        })
        .collect()
}

fn has_required_fields(record: &HashMap<&str, &str>) -> bool {
    REQUIRED_FIELDS.iter().all(|k| record.contains_key(k))
}

pub fn day4_part1(input: String) -> u64 {
    input.split("\n\n")
        .map(parse_record)
        .filter(has_required_fields)
        .count() as u64
}

fn is_valid(record: &HashMap<&str, &str>) -> bool {
    has_required_fields(record) &&
        record.keys()
            .all(|k|
                VALIDATORS.get(k).unwrap()(record.get(k).unwrap()))
}

pub fn day4_part2(input: String) -> u64 {
    input.split("\n\n")
        .map(parse_record)
        .filter(is_valid)
        .count() as u64
}