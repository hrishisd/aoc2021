use itertools::Itertools;

fn main() {
    let mut grid = include_str!("../../input.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("illegal digit"))
                .collect()
        })
        .collect::<Vec<Vec<u32>>>();
    println!("{}", part1(&grid));
    println!("{}", part2(&mut grid));
}

fn part1(grid: &[Vec<u32>]) -> u32 {
    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|(x, y)| {
            neighbors(*x, *y, grid)
                .into_iter()
                .all(|(x_, y_)| grid[*x][*y] < grid[x_][y_])
        })
        .map(|(x, y)| grid[x][y] + 1)
        .sum()
}

fn part2(grid: &mut [Vec<u32>]) -> u32 {
    let mut basin_sizes = Vec::new();
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            if grid[x][y] != 9 {
                basin_sizes.push(explore(grid, x, y));
            }
        }
    }
    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn explore(grid: &mut [Vec<u32>], x: usize, y: usize) -> u32 {
    if grid[x][y] == 9 {
        0
    } else {
        grid[x][y] = 9;
        1 + neighbors(x, y, grid)
            .iter()
            .map(|(x_, y_)| explore(grid, *x_, *y_))
            .sum::<u32>()
    }
}

fn neighbors(x: usize, y: usize, grid: &[Vec<u32>]) -> Vec<(usize, usize)> {
    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .map(move |(dx, dy)| (x as i32 + dx, y as i32 + dy))
        .filter(|(x, y)| {
            0 <= *x && (*x as usize) < grid.len() && 0 <= *y && (*y as usize) < grid[0].len()
        })
        .map(|(x, y)| (x as usize, y as usize))
        .collect()
}
