use std::io::{self, Read};

use clap::Clap;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

#[derive(Clap)]
struct Opts {
    #[clap(short)]
    day: u8,
    #[clap(short)]
    part: u8
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let result = match (opts.day, opts.part) {
        (1, 1) => Ok(day1::day1_part1(input)),
        (1, 2) => Ok(day1::day1_part2(input)),
        (2, 1) => Ok(day2::day2_part1(input)),
        (2, 2) => Ok(day2::day2_part2(input)),
        (3, 1) => Ok(day3::day3_part1(input)),
        (3, 2) => Ok(day3::day3_part2(input)),
        (4, 1) => Ok(day4::day4_part1(input)),
        (4, 2) => Ok(day4::day4_part2(input)),
        (5, 1) => Ok(day5::day5_part1(input)),
        (5, 2) => Ok(day5::day5_part2(input)),
        (6, 1) => Ok(day6::day6_part1(input)),
        (6, 2) => Ok(day6::day6_part2(input)),
        (7, 1) => Ok(day7::day7_part1(input)),
        (7, 2) => Ok(day7::day7_part2(input)),
        _ => Err("Invalid day / part combination")
    };

    match result {
        Ok(r) => println!("Answer: {}", r),
        Err(msg) => println!("Error: {}", msg)
    }
}
