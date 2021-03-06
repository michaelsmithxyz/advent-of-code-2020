use std::collections::HashSet;

use itertools::Itertools;

pub fn day1_part1(input: String) -> i64 {
    let values = input
        .split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<HashSet<_>>();

    let complements = values
        .iter()
        .map(|v| 2020 - v)
        .collect::<HashSet<_>>();

    let m = values.intersection(&complements).nth(0).unwrap();
    m * (2020 - m)
}

pub fn day1_part2(input: String) -> i64 {
    input
        .split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .combinations(3)
        .find(|vals| vals[0] + vals[1] + vals[2] == 2020)
        .map(|vals| vals[0] * vals[1] * vals[2])
        .unwrap()
}