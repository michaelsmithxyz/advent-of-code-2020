use std::collections::HashSet;
use itertools::Itertools;
use std::hash::Hash;
use std::cmp::Eq;

fn intersect<T: Hash + Eq + Copy>(a: HashSet<T>, b: HashSet<T>) -> HashSet<T> {
    a.intersection(&b).into_iter().cloned().collect()
}

pub fn day6_part1(input: String) -> i64 {
    input.split("\n\n")
        .map(|group| group.chars()
            .filter(char::is_ascii_alphabetic)
            .unique()
            .count() as i64)
        .sum()
}

pub fn day6_part2(input: String) -> i64 {
    input.split("\n\n")
        .map(|group| group.lines()
            .map(|l| l.chars().collect::<HashSet<_>>())
            .fold((b'a'..=b'z').map(char::from).collect::<HashSet<_>>(), intersect)
            .len() as i64)
        .sum()
}