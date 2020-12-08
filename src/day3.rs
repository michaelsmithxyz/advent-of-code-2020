use std::vec::Vec;

pub fn read_topo(input: String) -> Vec<Vec<char>> {
    input.lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn measure_slope(topo: &Vec<Vec<char>>, right: usize, down: usize) -> i64 {
    topo
        .iter()
        .step_by(down)
        .enumerate()
        .map(|(i, line)| {
            let rotated = {
                let mut copy = line.clone();
                copy.rotate_left((i * right) % line.len());
                copy
            };
            if rotated[0] == '#' { 1 } else { 0 }
        })
        .sum()
}

pub fn day3_part1(input: String) -> i64 {
    measure_slope(&read_topo(input), 3, 1)
}

pub fn day3_part2(input: String) -> i64 {
    let topo = read_topo(input);
    measure_slope(&topo, 1, 1)
        * measure_slope(&topo, 3, 1)
        * measure_slope(&topo, 5, 1)
        * measure_slope(&topo, 7, 1)
        * measure_slope(&topo, 1, 2)
}