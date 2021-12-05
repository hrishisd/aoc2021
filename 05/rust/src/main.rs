use scan_fmt::scan_fmt;
use std::ops::Div;

type Segment = ((i16, i16), (i16, i16));
const N: usize = 1000;

fn main() {
    let input = include_str!("../../input.txt");
    let line_segments = input.lines().map(parse_line_segment).collect::<Vec<_>>();
    println!("{}", part1(&line_segments));
    println!("{}", part2(&line_segments));
}

fn part1(line_segments: &[Segment]) -> usize {
    solve_num_overlaps(line_segments, |((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
}

fn part2(line_segments: &[Segment]) -> usize {
    solve_num_overlaps(line_segments, |_| true)
}

fn solve_num_overlaps(line_segments: &[Segment], predicate: fn(Segment) -> bool) -> usize {
    let mut grid = [[0; N]; N];
    let mut num_overlaps = 0;
    line_segments
        .iter()
        .filter(|&&segment| predicate(segment))
        .for_each(|((mut x1, mut y1), (x2, y2))| {
            let vertical_dist = y2 - y1;
            let horizontal_dist = x2 - x1;
            let max_dist = vertical_dist.abs().max(horizontal_dist.abs());
            let vertical_step = vertical_dist.div(max_dist);
            let horizontal_step = horizontal_dist.div(max_dist);
            while x1 != *x2 || y1 != *y2 {
                num_overlaps += set_grid(&mut grid, x1, y1);
                x1 += horizontal_step;
                y1 += vertical_step;
            }
            num_overlaps += set_grid(&mut grid, x1, y1);
        });
    num_overlaps
}

fn set_grid(grid: &mut [[i16; N]], x: i16, y: i16) -> usize {
    let (x, y) = (x as usize, y as usize);
    if grid[x][y] == 0 {
        grid[x][y] = 1;
        0
    } else if grid[x][y] == 1 {
        grid[x][y] = 2;
        1
    } else {
        0
    }
}

fn parse_line_segment(line: &str) -> Segment {
    let (x1, y1, x2, y2) =
        scan_fmt!(line, "{},{} -> {},{}", i16, i16, i16, i16).expect("invalid input");
    ((x1, y1), (x2, y2))
}
