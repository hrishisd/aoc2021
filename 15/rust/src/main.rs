use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let grid: Vec<Vec<i32>> = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("invalid input") as i32)
                .collect()
        })
        .collect();
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    println!(
        "part 1: {}",
        shortest_path(|r, c| grid[r][c], num_rows, num_cols)
    );
    let extended_grid_lookup = |extended_grid_row: usize, extended_grid_col: usize| {
        let (row_offset, row) = (extended_grid_row / num_rows, extended_grid_row % num_rows);
        let (col_offset, col) = (extended_grid_col / num_cols, extended_grid_col % num_cols);
        let val = grid[row][col];
        (val - 1 + row_offset as i32 + col_offset as i32) % 9 + 1
    };
    println!(
        "part 2: {}",
        shortest_path(extended_grid_lookup, 5 * num_rows, 5 * num_cols)
    );
}

fn shortest_path<F>(grid_val: F, num_rows: usize, num_cols: usize) -> i32
where
    F: Fn(usize, usize) -> i32,
{
    let in_range = |row, col| row < num_rows && col < num_cols;
    let neighbors = |(r, c): (usize, usize)| {
        let (r, c) = (r as i32, c as i32);
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(move |(dr, dc)| (r + dr, c + dc))
            .filter(|(r, c)| *r >= 0 && *c >= 0 && *r < num_rows as i32 && *c < num_cols as i32)
            .map(|(r, c)| (r as usize, c as usize))
    };
    let mut distances = vec![vec![i32::MAX; num_cols]; num_rows];
    distances[0][0] = 0;
    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, (0, 0))));
    loop {
        let Reverse((dist, (row, col))) = pq.pop().expect("illegal state");
        for (r, c) in neighbors((row, col)) {
            if in_range(r, c) {
                let new_dist = grid_val(r, c) + dist;
                let old_dist = &mut distances[r][c];
                if new_dist < *old_dist {
                    *old_dist = new_dist;
                    pq.push(Reverse((new_dist, (r, c))));
                }
            }
        }
        if row == num_rows - 1 && col == num_cols - 1 {
            break distances[row][col];
        }
    }
}
