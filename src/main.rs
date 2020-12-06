use std::fs;

use clap::Clap;

mod day1;
mod day2;
mod day3;

#[derive(Clap)]
struct Opts {
    #[clap(short)]
    day: u8,
    #[clap(short)]
    part: u8,
    #[clap(short)]
    input: String
}

fn main() {
    let opts: Opts = Opts::parse();

    let input = fs::read_to_string(opts.input)
        .expect("Unable to read input file!");

    let result = match (opts.day, opts.part) {
        (1, 1) => Ok(day1::day1_part1(input)),
        (1, 2) => Ok(day1::day1_part2(input)),
        (2, 1) => Ok(day2::day2_part1(input)),
        (2, 2) => Ok(day2::day2_part2(input)),
        (3, 1) => Ok(day3::day3_part1(input)),
        (3, 2) => Ok(day3::day3_part2(input)),
        _ => Err("Invalid day / part combination")
    };

    match result {
        Ok(r) => println!("Answer: {}", r),
        Err(msg) => println!("Error: {}", msg)
    }
}
