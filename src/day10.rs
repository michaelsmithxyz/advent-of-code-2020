use std::collections::HashMap;

pub fn day10_part1(input: String) -> i64 {
    let mut sorted: Vec<_> = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect();

    sorted.sort();

    let (ones, threes) = sorted
        .as_slice()
        .windows(2)
        .map(|w| w[1] - w[0])
        .fold((1 as i64, 1 as i64), |(ones, threes), el|
            match el {
                1 => (ones + 1, threes),
                3 => (ones, threes + 1),
                _ => (ones, threes)
            });
    ones * threes
}

fn combinations(adapters: &[i64], start: i64, results: &mut HashMap<i64, i64>) -> i64 {
    if results.contains_key(&start)  {
        *results.get(&start).unwrap()
    } else {
        let r = adapters
            .iter()
            .filter(|n| **n < start && (start - **n <= 3))
            .map(|n| combinations(adapters, *n, results))
            .sum();
        results.insert(start, r);
        r + if start <= 3 { 1 } else { 0 }
    }
}

pub fn day10_part2(input: String) -> i64 {
    let mut sorted: Vec<_> = input
    .lines()
    .map(|l| l.parse::<i64>().unwrap())
    .collect();

    sorted.sort();

    combinations(
        &sorted,
        sorted[sorted.len() - 1],
        &mut HashMap::new())
}