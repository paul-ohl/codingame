use std::io;
use std::str::SplitWhitespace;

mod matrix;
use matrix::{Instruction, Grid};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

fn parse_instruction(instruction: &str) -> Instruction {
    let get_param_char = |mut si: SplitWhitespace| si.next().unwrap().chars().next().unwrap();
    let get_param_num = |mut si: SplitWhitespace| si.next().unwrap().to_owned().parse().unwrap();

    let mut instruction_iter = instruction.split_whitespace();

    match &*instruction_iter.next().unwrap().to_lowercase() {
        "cs" => Instruction::ClearScreen(get_param_char(instruction_iter)),
        "fd" => Instruction::Forward(get_param_num(instruction_iter)),
        "rt" => Instruction::Right(get_param_num(instruction_iter)),
        "lt" => Instruction::Left(get_param_num(instruction_iter)),
        "pu" => Instruction::PenUp,
        "pd" => Instruction::PenDown,
        "setpc" => Instruction::SetChar(get_param_char(instruction_iter)),
        _ => panic!("Instruction not recognized"),
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let n = parse_input!(input_line, i32);
    let mut input: Vec<String> = Vec::with_capacity(n as usize);
    for _ in 0..n as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let line = input_line.trim_matches('\n').to_string();
        input.push(line);
    }
    let instructions: Vec<Instruction> = input
        .iter()
        .flat_map(|s| s.split(';').collect::<Vec<&str>>())
        .filter(|s| !s.is_empty())
        .map(parse_instruction)
        .collect();
    let mut grid = Grid::new();
    for instruction in instructions {
        grid.execute(instruction);
    }
    grid.display_grid();
}

