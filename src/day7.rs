use std::vec::Vec;
use std::collections::HashSet;
use std::hash::Hash;

use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref BAG_REGEX: Regex = Regex::new(r##"(\w+ \w+) bags contain"##).unwrap();
}

lazy_static! {
    static ref ITEMS_REGEX: Regex = Regex::new(r##"(\d+) (\w+ \w+) bags?"##).unwrap();
}

#[derive(Debug, Clone)]
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

fn union<T: Hash + Eq + Clone>(a: HashSet<T>, b: HashSet<T>) -> HashSet<T> {
    a.union(&b).into_iter().cloned().collect()
}

fn get_inner_bags(rules: &Vec<BagRule>, start: &str) -> HashSet<String> {
    let start_rule = rules.iter().find(|r| *r.color == *start).unwrap();
    start_rule.items
        .iter()
        .map(|(color, _)| {
            let mut children = get_inner_bags(rules, color);
            children.insert(color.clone());
            children
        })
        .fold(HashSet::<String>::new(), union)
}

pub fn day7_part1(input: String) -> u64 {
    let rules: Vec<BagRule> = input.lines().map(parse_rule).collect();
    rules.clone()
        .into_iter()
        .map(|r| get_inner_bags(&rules, &r.color))
        .filter(|s| s.contains("shiny gold"))
        .count() as u64
}

fn count_contents(rules: &Vec<BagRule>, start: &str) -> u64 {
    let start_rule = rules.iter().find(|r| *r.color == *start).unwrap();
    start_rule.items
        .iter()
        .map(|(color, count)| (*count as u64) * count_contents(rules, color))
        .sum::<u64>() + 1
}

pub fn day7_part2(input: String) -> u64 {
    let rules: Vec<BagRule> = input.lines().map(parse_rule).collect();
    count_contents(&rules, "shiny gold") - 1
}