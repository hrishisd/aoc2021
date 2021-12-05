use anyhow::anyhow;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Command {
    Forward(i32),
    Vertical(i32),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let (direction, num) = s
            .split_ascii_whitespace()
            .next_tuple()
            .ok_or(anyhow!("invalid command string"))?;
        let num = num.parse()?;
        match direction {
            "forward" => anyhow::Ok(Command::Forward(num)),
            "up" => anyhow::Ok(Command::Vertical(-num)),
            "down" => anyhow::Ok(Command::Vertical(num)),
            _ => Err(anyhow!("Invalid direction: {}", direction)),
        }
    }
}

fn solve_part1(commands: &[Command]) -> i32 {
    let (final_x, final_y) = commands.iter().fold((0, 0), |(x, y), cmd| match cmd {
        Command::Forward(dx) => (x + dx, y),
        Command::Vertical(dy) => (x, y + dy),
    });
    final_x * final_y
}

fn solve_part2(commands: &[Command]) -> i32 {
    let (_, x, depth) = commands
        .iter()
        .fold((0, 0, 0), |(aim, x, depth), cmd| match cmd {
            Command::Forward(val) => (aim, x + val, depth + aim * val),
            Command::Vertical(val) => (aim + val, x, depth),
        });
    x * depth
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../input.txt");
    let commands = input
        .lines()
        .map(Command::from_str)
        .collect::<anyhow::Result<Vec<_>>>()?;
    println!("part 1: {}", solve_part1(&commands));
    println!("part 2: {}", solve_part2(&commands));
    Ok(())
}
