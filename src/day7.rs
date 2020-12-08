use std::vec::Vec;

use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref BAG_REGEX: Regex = Regex::new(r##"(\w+ \w+) bags contain"##).unwrap();
}

lazy_static! {
    static ref ITEMS_REGEX: Regex = Regex::new(r##"(\d+) (\w+ \w+) bags?"##).unwrap();
}

#[derive(Debug)]
struct BagRule {
    color: String,
    items: Vec<(String, u8)>
}

fn parse_rule(line: &str) -> BagRule {
    BagRule {
        color: BAG_REGEX.captures(line).unwrap()[1].to_string(),
        items: ITEMS_REGEX.captures_iter(line)
            .map(|cap| (cap[2].into(), cap[1].parse::<u8>().unwrap()))
            .collect()
    }
}

fn has_inner_bag(rules: &Vec<BagRule>, start: &str, find: &str) -> bool {
    let start_rule = rules.iter().find(|r| *r.color == *start).unwrap();
    start_rule.items
        .iter()
        .map(|(color, _)| color == find || has_inner_bag(rules, &color, find))
        .any(|b| b)
}

pub fn day7_part1(input: String) -> u64 {
    let rules: Vec<BagRule> = input.lines().map(parse_rule).collect();
    rules.iter()
        .filter(|r|
            has_inner_bag(&rules, &r.color, "shiny gold"))
        .count() as u64
}

fn count_contents(rules: &Vec<BagRule>, start: &str) -> u64 {
    let start_rule = rules.iter().find(|r| *r.color == *start).unwrap();
    1 + start_rule.items
        .iter()
        .map(|(color, count)| (*count as u64) * count_contents(rules, color))
        .sum::<u64>()
}

pub fn day7_part2(input: String) -> u64 {
    let rules: Vec<BagRule> = input.lines().map(parse_rule).collect();
    count_contents(&rules, "shiny gold") - 1
}