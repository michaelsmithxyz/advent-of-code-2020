use std::collections::HashSet;
use std::vec::Vec;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Nop(i64),
    Acc(i64),
    Jmp(i64)
}

#[derive(Debug, Copy, Clone)]
struct State {
    line: usize,
    acc: i64
}

fn parse(line: &str) -> Instruction {
    let normalized = line.replace(" ", "");
    let (op, arg) = normalized.split_at(3);
    let parsed_arg = arg.parse::<i64>().unwrap();
    match op {
        "nop" => Instruction::Nop(parsed_arg),
        "acc" => Instruction::Acc(parsed_arg),
        "jmp" => Instruction::Jmp(parsed_arg),
        _ => panic!("Unknown instruction!")
    }
}

fn step(state: &State, inst: &Instruction) -> State {
    match inst {
        Instruction::Nop(_) => State {
            line: state.line + 1,
            acc: state.acc
        },
        Instruction::Acc(v) => State {
            line: state.line + 1,
            acc: state.acc + v
        },
        Instruction::Jmp(v) => State {
            line: ((state.line as i64) + v) as usize,
            acc: state.acc
        }
    }
}

fn run(state: &State, instructions: &Vec<Instruction>) -> Result<i64, i64> {
    let mut seen: HashSet<usize> = HashSet::new();
    let mut current_state = *state;
    loop {
        if seen.contains(&current_state.line) {
            return Err(current_state.acc)
        }
        if current_state.line >= instructions.len() {
            return Ok(current_state.acc)
        }
        let inst = instructions[current_state.line as usize];
        seen.insert(current_state.line);
        current_state = step(&current_state, &inst);
    }
}

pub fn day8_part1(input: String) -> i64 {
    let initial_state = State {
        line: 0,
        acc: 0
    };
    let instructions = input.lines().map(parse).collect();
    run(&initial_state, &instructions).expect_err("Program did not infinitely loop")
}

pub fn day8_part2(input: String) -> i64 {
    let initial_state = State {
        line: 0,
        acc: 0
    };
    let instructions: Vec<Instruction> = input.lines().map(parse).collect();
    instructions.iter()
        // Match instructions w/ their line numbers
        .enumerate()
        // Identify replaceable instructions
        .filter(|(_, inst)| match **inst {
            Instruction::Jmp(_) | Instruction::Nop(_) => true,
            _ => false
        })
        // Make the replacement
        .map(|(idx, inst)| (idx, match inst {
            Instruction::Jmp(x) => Instruction::Nop(*x),
            Instruction::Nop(x) => Instruction::Jmp(*x),
            _ => *inst
        }))
        // Splice the new instruction into the original program and run it,
        // stopping at the first successful run
        .find_map(|(idx, inst)| {
            let mut new_instructions = instructions.clone();
            std::mem::replace(&mut new_instructions[idx], inst);
            run(&initial_state, &new_instructions).ok()
        })
        .unwrap()
}