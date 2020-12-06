use std::vec::Vec;
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref LINE_PATTERN: Regex = Regex::new(r##"(\d+)\-(\d+)\s+([a-z]):\s+([a-z]+)"##).unwrap();
}
const CAPTURE_GROUP_MIN: usize = 1;
const CAPTURE_GROUP_MAX: usize = 2;
const CAPTURE_GROUP_CHAR: usize = 3;
const CAPTURE_GROUP_PASSWORD: usize = 4;

type Row = (usize, usize, char, String);

fn parse_row(row: &str) -> Row {
    let captures = LINE_PATTERN.captures(row).unwrap();

    let min = captures.get(CAPTURE_GROUP_MIN)
        .map(|v| v.as_str().parse::<usize>().unwrap())
        .unwrap();
    let max = captures.get(CAPTURE_GROUP_MAX)
        .map(|v| v.as_str().parse::<usize>().unwrap())
        .unwrap();
    let character = captures.get(CAPTURE_GROUP_CHAR)
        .map(|v| v.as_str().chars().nth(0).unwrap())
        .unwrap();
    let password = captures.get(CAPTURE_GROUP_PASSWORD)
        .map(|v| v.as_str())
        .unwrap();

    (min, max, character, password.into())
}

fn is_valid_part1(row: &Row) -> bool {
    let (min, max, character, password) = row.clone();
    let occurances = password
        .matches(character)
        .count();
    occurances >= min && occurances <= max
}

pub fn day2_part1(input: String) {
    let count = input.lines()
        .map(parse_row)
        .filter(is_valid_part1)
        .count();
    println!("{}", count);
}

fn is_valid_part2(row: &Row) -> bool {
    let (fst, snd, character, password) = row.clone();
    let pwd_chars = password.chars().collect::<Vec<_>>();
    let matches_fst = pwd_chars[fst - 1] == character;
    let matches_snd = pwd_chars[snd - 1] == character;
    matches_fst ^ matches_snd
}

pub fn day2_part2(input: String) {
    let count = input.lines()
        .map(parse_row)
        .filter(is_valid_part2)
        .count();
    println!("{}", count);
}