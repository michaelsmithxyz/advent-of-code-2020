use std::collections::HashSet;

const ROW_MAX: u8 = 127;
const COL_MAX: u8 = 7;

#[derive(Debug)]
enum Rows {
    Single(u8),
    Range(u8, u8)
}

impl Rows {
    fn unwrap(&self) -> u8 {
        match self {
            Rows::Single(v) => *v,
            _ => panic!("Unwrapped range for Row")
        }
    }
}

#[derive(Debug)]
enum Reduction {
    Lower,
    Upper
}

fn parse(line:  &str) -> impl Iterator<Item = Reduction> +'_ {
    line.chars()
        .into_iter()
        .map(|c| match c {
            'F' | 'L'  => Reduction::Lower,
            'B' | 'R' => Reduction::Upper,
            _ => panic!("Invalid input")
    })
}

fn reduce(rows: Rows, reduction: Reduction) -> Rows {
    let (current_min, current_max) = match rows {
        Rows::Range(min, max) => (min, max),
        Rows::Single(_) => return rows
    };
    let midpoint = ((current_max - current_min - 1) / 2) + current_min;
    let (min, max) = match reduction {
        Reduction::Lower => (current_min, midpoint),
        Reduction::Upper => (midpoint + 1, current_max)
    };
    if min == max { Rows::Single(min) } else { Rows::Range(min, max) }
}

fn seat_location_from_pattern(pattern: &str) -> (u8, u8) {
    let (row_pattern, col_pattern) = pattern.split_at(7);
    let row = parse(row_pattern)
        .fold(Rows::Range(0, ROW_MAX), reduce)
        .unwrap();
    let col = parse(col_pattern)
        .fold(Rows::Range(0, COL_MAX), reduce)
        .unwrap();
    (row, col)
}

fn id((row, col): (u8, u8)) -> u16 {
    (row as u16) * 8 + (col as u16)
}

pub fn day5_part1(input: String) -> u64 {
    input.lines()
        .map(|l| {
            id(seat_location_from_pattern(l))
        })
        .max()
        .unwrap() as u64
}

pub fn day5_part2(input: String) -> u64 {
    let mut continuous_ids: Vec<_> = input.lines()
        .map(seat_location_from_pattern)
        // Seats may be missing from either the very front or back of the plane,
        // so exclude those rows since we know our seat isn't there
        .filter(|(r, _)| *r != 0 && *r != ROW_MAX)
        .map(id)
        .collect();
    continuous_ids.sort();

    let full_range: HashSet<_> = (continuous_ids[0]..continuous_ids[continuous_ids.len() - 1]).collect();
    let present_values: HashSet<_> = continuous_ids.into_iter().collect();
    *full_range.difference(&present_values).nth(0).unwrap() as u64
}