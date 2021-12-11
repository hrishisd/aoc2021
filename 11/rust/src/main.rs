use itertools::Itertools;

type Grid = [[u8; 10]; 10];

fn main() {
    let grid = parse_grid(include_str!("../../input.txt"));
    println!("part 1: {}", part1(grid));
    println!("part 2: {}", part2(grid));
}

fn part1(mut grid: Grid) -> i32 {
    let mut result = 0;
    for _ in 0..100 {
        result += step(&mut grid);
    }
    result
}

fn part2(mut grid: Grid) -> i32 {
    let mut i = 1;
    loop {
        if step(&mut grid) == 100 {
            return i;
        }
        i += 1;
    }
}

fn step(grid: &mut Grid) -> i32 {
    grid.iter_mut().flatten().for_each(|i| *i += 1);
    let mut total_num_flashes = 0;
    loop {
        let flashes = single_pass(grid);
        if flashes == 0 {
            return total_num_flashes;
        }
        total_num_flashes += flashes;
    }
}

fn single_pass(grid: &mut Grid) -> i32 {
    let mut flashes = 0;
    for x in 0..10 {
        for y in 0..10 {
            if grid[x][y] > 9 {
                grid[x][y] = 0;
                flashes += 1;
                for (x, y) in neighbors(x, y) {
                    if grid[x][y] > 0 {
                        grid[x][y] += 1;
                    }
                }
            }
        }
    }
    flashes
}

fn neighbors(x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    let t = [-1, 0, 1];
    t.into_iter()
        .cartesian_product(t.into_iter())
        .filter(|(dx, dy)| (*dx, *dy) != (0, 0))
        .map(move |(dx, dy)| (x as i32 + dx, y as i32 + dy))
        .filter(|(x, y)| 0 <= *x && *x < 10 && 0 <= *y && *y < 10)
        .map(|(x, y)| (x as usize, y as usize))
}

fn parse_grid(input: &str) -> Grid {
    let mut result = [[0; 10]; 10];
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            result[x][y] = c.to_digit(10).expect("illegal digit") as u8;
        }
    }
    result
}
