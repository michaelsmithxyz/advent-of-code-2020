use std::fs;

use clap::Clap;

mod day1;


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

    match (opts.day, opts.part) {
        (1, 1) => day1::day1_part1(input),
        (1, 2) => day1::day1_part2(input),
        _ => println!("Invalid day / part combination")
    }
}
