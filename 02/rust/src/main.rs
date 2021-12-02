use anyhow::anyhow;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (direction, num) = s
            .split_ascii_whitespace()
            .next_tuple()
            .ok_or(anyhow!("invalid command string"))?;
        let num = num.parse::<i32>()?;
        match direction {
            "forward" => anyhow::Ok(Command::Forward(num)),
            "up" => anyhow::Ok(Command::Up(num)),
            "down" => anyhow::Ok(Command::Down(num)),
            _ => Err(anyhow!("Invalid direction: {}", direction)),
        }
    }
}

fn solve_part1(commands: &[Command]) -> i32 {
    let (final_x, final_y) = commands.iter().fold((0, 0), |(x, y), cmd| match cmd {
        Command::Forward(dx) => (x + dx, y),
        Command::Up(dy) => (x, y - dy),
        Command::Down(dy) => (x, y + dy),
    });
    final_x * final_y
}

fn solve_part2(commands: &[Command]) -> i32 {
    let mut aim = 0;
    let mut x = 0;
    let mut depth = 0;
    for cmd in commands {
        match cmd {
            Command::Forward(val) => {
                x += val;
                depth += aim * val;
            }
            Command::Up(val) => aim -= val,
            Command::Down(val) => aim += val,
        }
    }
    x * depth
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../input.txt");
    let commands = input
        .lines()
        .map(|s| s.parse::<Command>())
        .collect::<anyhow::Result<Vec<Command>>>()?;
    println!("part 1: {}", solve_part1(&commands));
    println!("part 2: {}", solve_part2(&commands));
    Ok(())
}
