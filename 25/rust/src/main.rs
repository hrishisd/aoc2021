use std::fmt::Debug;

use itertools::Itertools;
use Cell::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    EastFacing,
    SouthFacing,
    Empty,
}

impl Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EastFacing => write!(f, ">"),
            Self::SouthFacing => write!(f, "v"),
            Self::Empty => write!(f, "."),
        }
    }
}

fn main() {
    let mut grid = include_str!("../../input.txt")
        .lines()
        .map(parse_line)
        .collect_vec();
    println!("{}", part1(&mut grid));
    println!("Hello, world!");
}

fn part1(grid: &mut [Vec<Cell>]) -> usize {
    for i in 1..usize::MAX {
        if !step(grid) {
            return i;
        }
    }
    panic!("reached usize max without finding terminal state");
}

fn step(grid: &mut [Vec<Cell>]) -> bool {
    let nrows = grid.len();
    let ncols = grid[0].len();
    let mut moved = false;
    for row in grid.iter_mut() {
        let mut to_move = Vec::new();
        for col_idx in 0..ncols {
            if row[col_idx] == EastFacing && row[(col_idx + 1) % ncols] == Empty {
                to_move.push(col_idx);
            }
        }
        for col_idx in to_move {
            row[col_idx] = Empty;
            row[(col_idx + 1) % ncols] = EastFacing;
            moved = true;
        }
    }

    for col in 0..ncols {
        let mut to_move = Vec::new();
        for row in 0..nrows {
            if grid[row][col] == SouthFacing && grid[(row + 1) % nrows][col] == Empty {
                to_move.push(row);
            }
        }
        for row in to_move {
            grid[row][col] = Empty;
            grid[(row + 1) % nrows][col] = SouthFacing;
            moved = true;
        }
    }
    moved
}

fn parse_line(line: &str) -> Vec<Cell> {
    line.chars()
        .map(|c| match c {
            '>' => EastFacing,
            'v' => SouthFacing,
            '.' => Empty,
            _ => {
                panic!("Invalid input");
            }
        })
        .collect_vec()
}

fn display(grid: &[Vec<Cell>]) {
    for row in grid {
        for cell in row {
            print!("{:?}", *cell);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use crate::{display, parse_line, step};

    #[test]
    fn test_step() {
        let mut grid = [parse_line("...>>>>>...")];
        display(&grid);
        step(&mut grid);
        display(&grid);
    }
}
