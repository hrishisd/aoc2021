use anyhow::anyhow;
use core::panic;
use std::{collections::HashSet, fmt::Debug, ops::Add, str::FromStr};

use itertools::{enumerate, Itertools};
use Instruction::*;
type ProgramState = [i32; 4];

#[derive(Clone, Copy)]
enum Instruction {
    Inp(usize),
    Add(usize, usize),
    Addi(usize, i32),
    Mul(usize, usize),
    Muli(usize, i32),
    Div(usize, usize),
    Divi(usize, i32),
    Mod(usize, usize),
    Modi(usize, i32),
    Eq(usize, usize),
    Eqi(usize, i32),
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn var_name(i: usize) -> &'static str {
            match i {
                0 => "w",
                1 => "x",
                2 => "y",
                3 => "z",
                _ => panic!("illegal state"),
            }
        }
        match self {
            Self::Inp(l) => f.debug_tuple("Inp").field(&var_name(*l)).finish(),
            Self::Add(l, r) => f
                .debug_tuple("Add")
                .field(&var_name(*l))
                .field(&var_name(*r))
                .finish(),
            Self::Addi(l, r) => f.debug_tuple("Add").field(&var_name(*l)).field(r).finish(),
            Self::Mul(l, r) => f
                .debug_tuple("Mul")
                .field(&var_name(*l))
                .field(&var_name(*r))
                .finish(),
            Self::Muli(l, r) => f.debug_tuple("Mul").field(&var_name(*l)).field(r).finish(),
            Self::Div(l, r) => f
                .debug_tuple("Div")
                .field(&var_name(*l))
                .field(&var_name(*r))
                .finish(),
            Self::Divi(l, r) => f.debug_tuple("Div").field(&var_name(*l)).field(r).finish(),
            Self::Mod(l, r) => f
                .debug_tuple("Mod")
                .field(&var_name(*l))
                .field(&var_name(*r))
                .finish(),
            Self::Modi(l, r) => f.debug_tuple("Mod").field(&var_name(*l)).field(r).finish(),
            Self::Eq(l, r) => f
                .debug_tuple("Eq")
                .field(&var_name(*l))
                .field(&var_name(*r))
                .finish(),
            Self::Eqi(l, r) => f.debug_tuple("Eq").field(&var_name(*l)).field(r).finish(),
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let to_register_idx = |s: &str| match s {
            "w" => Ok(0),
            "x" => Ok(1),
            "y" => Ok(2),
            "z" => Ok(3),
            _ => Err(anyhow!("invalid register: {:?}", s)),
        };

        let words = s.split_ascii_whitespace().collect_vec();
        if words.len() == 2 {
            match &words[..2] {
                &["inp", arg] => Ok(Inp(to_register_idx(arg)?)),
                _ => Err(anyhow!("invalid input")),
            }
        } else {
            let l = to_register_idx(words[1])?;
            match to_register_idx(words[2]) {
                Ok(r) => match words[0] {
                    "add" => Ok(Add(l, r)),
                    "mul" => Ok(Mul(l, r)),
                    "div" => Ok(Div(l, r)),
                    "mod" => Ok(Mod(l, r)),
                    "eql" => Ok(Eq(l, r)),
                    _ => Err(anyhow!("invalid instruction")),
                },
                Err(_) => {
                    let num = i32::from_str(words[2])?;
                    match words[0] {
                        "add" => Ok(Addi(l, num)),
                        "mul" => Ok(Muli(l, num)),
                        "div" => Ok(Divi(l, num)),
                        "mod" => Ok(Modi(l, num)),
                        "eql" => Ok(Eqi(l, num)),
                        _ => Err(anyhow!("invalid instruction")),
                    }
                }
            }
        }
    }
}

fn main() -> anyhow::Result<()> {
    let program: Vec<Instruction> = include_str!("../../input.txt")
        .lines()
        .map(Instruction::from_str)
        .try_collect()?;
    find_input(&program)?;
    Ok(())
}

fn find_input(program: &[Instruction]) -> anyhow::Result<i64> {
    let mut valid_end_states = vec![HashSet::new(); 14];
    valid_end_states[13].insert(0);
    for (block_idx, block) in program.chunks(18).enumerate().skip(1).rev() {
        for z in 1..1_000_000 {
            for digit in 1..=9 {
                match run(block, [0, 0, 0, z], &[digit]) {
                    Ok([_, _, _, end_z]) if valid_end_states[block_idx].contains(&end_z) => {
                        valid_end_states[block_idx - 1].insert(z);
                    }
                    _ => {}
                }
            }
        }
        println!("{}: {}", block_idx, valid_end_states[block_idx].len());
    }

    let mut digits = [0; 14];
    let mut prev_z = 0;
    for (idx, block) in program.chunks(18).enumerate() {
        let (digit, z) = find_valid_digit(block, prev_z, &valid_end_states[idx]);
        digits[idx] = digit;
        prev_z = z;
    }
    println!("{}", digits.map(|d| d.to_string()).join(""));
    Ok(0)
}

fn find_valid_digit(block: &[Instruction], z: i32, valid_end_states: &HashSet<i32>) -> (i32, i32) {
    for digit in (1..=9) {
        match run(block, [0, 0, 0, z], &[digit]) {
            Ok([_, _, _, z]) if valid_end_states.contains(&z) => {
                return (digit, z);
            }
            _ => continue,
        }
    }
    panic!("didin't find valid digit");
}

fn run(
    program: &[Instruction],
    initial_state: ProgramState,
    inputs: &[i32],
) -> anyhow::Result<ProgramState> {
    let mut inputs = inputs.iter();
    let mut state = initial_state;
    for instruction in program.iter().copied() {
        match instruction {
            Inp(i) => {
                state[i] = *inputs.next().ok_or(anyhow!("ran out of inputs"))?;
            }
            Add(l, r) => {
                state[l] += state[r];
            }
            Addi(l, i) => {
                state[l] += i;
            }
            Mul(l, r) => {
                state[l] *= state[r];
            }
            Muli(l, i) => {
                state[l] *= i;
            }
            Div(l, r) => {
                if state[r] == 0 {
                    return Err(anyhow!("division by zero"));
                }
                state[l] /= state[r];
            }
            Divi(l, i) => {
                if i == 0 {
                    return Err(anyhow!("division by zero"));
                }
                state[l] /= i;
            }
            Mod(l, r) => {
                state[l] %= state[r];
            }
            Modi(l, i) => {
                state[l] %= i;
            }
            Eq(l, r) => {
                state[l] = if state[l] == state[r] { 1 } else { 0 };
            }
            Eqi(l, i) => {
                state[l] = if state[l] == i { 1 } else { 0 };
            }
        }
    }
    Ok(state)
}
