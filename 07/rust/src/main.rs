fn main() {
    let input = include_str!("../../input.txt");
    let mut nums: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().expect("invalid input"))
        .collect();
    nums.sort_unstable();
    println!("part 1: {}", part1(&nums));
    println!("part 2: {}", part2(&nums));
}

fn part1(xs: &[i32]) -> i32 {
    let median = (xs[xs.len() / 2] + xs[(xs.len() + 1) / 2]) / 2;
    xs.iter().map(|x| (x - median).abs()).sum()
}

fn part2(xs: &[i32]) -> i32 {
    let avg = xs.iter().sum::<i32>() / xs.len() as i32;
    xs.iter()
        .map(|x| (x - avg).abs())
        .map(|d| d * (d + 1) / 2)
        .sum()
}
