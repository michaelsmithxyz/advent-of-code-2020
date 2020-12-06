use std::vec::Vec;
use std::collections::HashMap;

static REQUIRED_FIELDS: [&str; 7] = [
    "byr",
    "iyr",
    "eyr",
    "hgt",
    "hcl",
    "ecl",
    "pid"
];

fn parse_record(input: &str) -> HashMap<&str, &str> {
    input
        .split_whitespace()
        .map(|fv| {
            let parts: Vec<&str> = fv.split(':').collect();
            (parts[0], parts[1])
        })
        .collect()
}

pub fn day4_part1(input: String) -> u64 {
    input.split("\n\n")
        .map(parse_record)
        .filter(|r| {
            REQUIRED_FIELDS.iter().all(|k| r.contains_key(k))
        })
        .count() as u64
}