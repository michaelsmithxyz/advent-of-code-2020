use std::cmp::Ordering;
use std::vec::Vec;

const WINDOW_SIZE: usize = 25;

fn split_window(window: &[i64]) -> (i64, &[i64]) {
    (window[window.len() - 1], &window[0..window.len() - 1])
}

/// Returns `None` if the given value and window are valid, otherwise `Some(v)`
fn identify_invalid_window((v, window): (i64, &[i64])) -> Option<i64> {
    let mut sorted = vec![0; window.len()];
    sorted.clone_from_slice(window);
    sorted.sort();

    let (mut low, mut high) = (0, sorted.len() - 1);
    while low != high {
        match (sorted[low] + sorted[high]).cmp(&v) {
            Ordering::Equal => return None,
            Ordering::Greater => high = high - 1,
            Ordering::Less => low = low + 1
        }
    }
    Some(v)
}

fn find_invalid_in_sequence(input: &[i64]) -> i64 {
    input.windows(WINDOW_SIZE + 1)
        .map(split_window)
        .find_map(identify_invalid_window)
        .unwrap()
}

pub fn day9_part1(input: String) -> i64 {
    find_invalid_in_sequence(
        input.lines()
            .map(|v| v.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
            .as_slice())
}

/// If `input` begins with a range of values that sum to `target`, return
/// the lowest and highest values in that range, otherwise `None`.
fn identify_sum_range(target: i64, input: &[i64]) -> Option<(i64, i64)> {
    /// Actually do the search, keeping track of the lowest and highest values
    /// we've seen so far
    fn do_identify_sum_range(target: i64, input: &[i64], low: i64, high: i64) -> Option<(i64, i64)> {
        let cur = input[0];
        match target.cmp(&cur) {
            Ordering::Equal => Some((target.min(low), target.max(high))),
            Ordering::Less => None,
            Ordering::Greater => do_identify_sum_range(
                target - input[0],
                &input[1..],
                low.min(cur),
                high.max(cur))
        }
    }
    do_identify_sum_range(target, input, i64::MAX, i64::MIN)
}

fn find_range(target: i64, input: &[i64]) -> (i64, i64) {
    (0..input.len())
        .find_map(|idx| identify_sum_range(target, &input[idx..]))
        .unwrap()
}

pub fn day9_part2(input: String) -> i64 {
    let input = input.lines()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let target = find_invalid_in_sequence(input.as_slice());
    let (low, high) = find_range(target, input.as_slice());
    low + high
}