use itertools::Itertools;
use std::num::ParseIntError;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../input.txt")
        .lines()
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<usize>, ParseIntError>>()?;
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    Ok(())
}

fn solve_part1(input: &[usize]) -> usize {
    input
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

fn solve_part2(input: &[usize]) -> usize {
    input
        .windows(3)
        .map(|window| window.iter().sum::<usize>())
        .tuple_windows()
        .filter(|(prev, next)| prev < next)
        .count()
}
