use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct BingoBoard([[Option<u8>; 5]; 5]);

impl FromStr for BingoBoard {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = [[Some(0u8); 5]; 5];
        for (i, line) in s.lines().enumerate() {
            for (j, num) in line.split_whitespace().enumerate() {
                result[i][j] = Some(num.parse::<u8>()?);
            }
        }
        Ok(BingoBoard(result))
    }
}

impl BingoBoard {
    fn mark(&mut self, num: u8) {
        for row in self.0.iter_mut() {
            for square in row.iter_mut() {
                if square == &Some(num) {
                    square.take();
                }
            }
        }
    }

    fn has_bingo(&self) -> bool {
        let grid = self.0;
        let horiontal_bingo = grid.iter().any(|row| row.iter().all(Option::is_none));
        let vertical_bingo = (0..5).any(|col| (0..5).all(|row| grid[row][col].is_none()));
        horiontal_bingo || vertical_bingo
    }

    fn get_unmarked_sum(&self) -> i32 {
        self.0
            .iter()
            .flat_map(|row| row.iter())
            .flat_map(Option::iter)
            .map(|&i| i as i32)
            .sum()
    }
}

fn score_game(mut boards: Vec<BingoBoard>, draws: &[u8]) -> Vec<i32> {
    let mut scores = Vec::new();
    for &draw in draws {
        for board in boards.iter_mut() {
            board.mark(draw);
        }
        for i in (0..boards.len()).rev() {
            if boards[i].has_bingo() {
                scores.push(draw as i32 * boards[i].get_unmarked_sum());
                boards.swap_remove(i);
            }
        }
    }
    scores
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../input.txt");
    let mut input = input.split("\n\n");
    let draws = input.next().expect("Invalid input");

    let draws = draws
        .split(',')
        .map(u8::from_str)
        .collect::<Result<Vec<u8>, _>>()?;

    let boards = input.map(BingoBoard::from_str).collect::<Result<_, _>>()?;

    let scores = score_game(boards, &draws);
    println!("part 1: {}", scores[0]);
    println!("part 2: {}", scores[scores.len() - 1]);
    Ok(())
}
