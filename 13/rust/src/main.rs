#![feature(hash_drain_filter)]
#![feature(iter_intersperse)]

use serde_scan::scan;
use std::{collections::HashSet, vec};

type Dot = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

impl Fold {
    fn fold(&self, (x, y): Dot) -> Dot {
        match self {
            Fold::Horizontal(pos) if y > *pos => (x, 2 * pos - y),
            Fold::Vertical(pos) if x > *pos => (2 * pos - x, y),
            _ => (x, y),
        }
    }
}

fn main() {
    let (dots, folds) = parse(include_str!("../../input.txt"));
    println!("{:?}", part1(&dots, folds[0]));
    part2(dots, &folds);
}

fn part1(dots: &HashSet<Dot>, f: Fold) -> usize {
    fold(dots, f).len()
}

fn part2(dots: HashSet<Dot>, fs: &[Fold]) {
    let mut dots = dots;
    for f in fs {
        dots = fold(&dots, *f);
    }
    display(&dots);
}

fn display(dots: &HashSet<Dot>) {
    let (width, height) = dots.iter().copied().fold((0, 0), |(max_x, max_y), (x, y)| {
        (max_x.max(x), max_y.max(y))
    });
    let mut grid = vec![vec!['.'; width + 1]; height + 1];
    for (x, y) in dots {
        grid[*y][*x] = '▋';
    }
    let s: String = grid.iter().intersperse(&vec!['\n']).flatten().collect();
    println!("{}", s);
}

fn fold(dots: &HashSet<Dot>, f: Fold) -> HashSet<(usize, usize)> {
    dots.iter().map(|&dot| f.fold(dot)).collect()
}

fn parse(input: &str) -> (HashSet<Dot>, Vec<Fold>) {
    let (dots, folds) = input.split_once("\n\n").expect("invalid input");
    let dots: HashSet<Dot> = dots
        .lines()
        .filter_map(|line| line.split_once(','))
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect();
    let folds: Vec<Fold> = folds
        .lines()
        .map(|line| {
            let (axis, pos): (&str, usize) =
                scan!("fold along {}={}" <- line).expect("invalid input");
            if axis == "x" {
                Fold::Vertical(pos)
            } else {
                Fold::Horizontal(pos)
            }
        })
        .collect();
    (dots, folds)
}
