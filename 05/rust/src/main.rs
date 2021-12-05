use scan_fmt::scan_fmt;

type Segment = ((i16, i16), (i16, i16));
const N: usize = 1000;

fn main() {
    let input = include_str!("../../input.txt");
    let line_segments = input.lines().map(parse_line_segment).collect::<Vec<_>>();
    println!("{}", part1(&line_segments));
    println!("{}", part2(&line_segments));
}

fn part1(line_segments: &[Segment]) -> usize {
    num_overlaps(
        line_segments
            .iter()
            .filter(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2)
            .copied(),
    )
}

fn part2(line_segments: &[Segment]) -> usize {
    num_overlaps(line_segments.iter().copied())
}

fn num_overlaps(line_segments: impl Iterator<Item = Segment>) -> usize {
    let mut grid = [[0; N]; N];
    for ((mut x1, mut y1), (x2, y2)) in line_segments {
        let dx = (x2 - x1).signum();
        let dy = (y2 - y1).signum();
        while (x1, y1) != (x2 + dx, y2 + dy) {
            grid[x1 as usize][y1 as usize] += 1;
            x1 += dx;
            y1 += dy;
        }
    }
    grid.iter().flatten().filter(|&&i| i > 1).count()
}

fn parse_line_segment(line: &str) -> Segment {
    let (x1, y1, x2, y2) =
        scan_fmt!(line, "{},{} -> {},{}", i16, i16, i16, i16).expect("invalid input");
    ((x1, y1), (x2, y2))
}
