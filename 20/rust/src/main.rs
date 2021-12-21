use std::collections::HashSet;

use bitvec::prelude::*;
use bitvec::{array::BitArray, bitarr};
use itertools::Itertools;

type Alg = BitArray<LocalBits, [usize; 8]>;

#[derive(Debug)]
struct Frame {
    top_row: i32,
    bottom_row: i32,
    left_col: i32,
    right_col: i32,
}

impl Frame {
    fn expand(&self) -> Frame {
        Frame {
            top_row: self.top_row - 2,
            bottom_row: self.bottom_row + 2,
            left_col: self.left_col - 2,
            right_col: self.right_col + 2,
        }
    }

    fn pixels(
        &self,
    ) -> itertools::Product<std::ops::RangeInclusive<i32>, std::ops::RangeInclusive<i32>> {
        (self.top_row..=self.bottom_row).cartesian_product(self.left_col..=self.right_col)
    }
}

#[derive(Debug)]
struct LazyImage {
    lit_pixels_in_frame: HashSet<(i32, i32)>,
    frame: Frame,
    lit_outside_frame: bool,
}

impl LazyImage {
    fn enhance(&mut self, alg: Alg) {
        let expanded_frame = self.frame.expand();
        let new_lit_pixels_in_frame: HashSet<(i32, i32)> = expanded_frame
            .pixels()
            .filter(|(row, col)| self.enhance_pixel(alg, *row, *col))
            .collect();
        let outside_window_value = if self.lit_outside_frame { 511 } else { 0 };
        let lit_outside_frame = *alg.get(outside_window_value).unwrap();
        self.frame = expanded_frame;
        self.lit_pixels_in_frame = new_lit_pixels_in_frame;
        self.lit_outside_frame = lit_outside_frame;
    }

    fn get_pixel(&self, row: i32, col: i32) -> bool {
        if row < self.frame.top_row
            || row > self.frame.bottom_row
            || col < self.frame.left_col
            || col > self.frame.right_col
        {
            self.lit_outside_frame
        } else {
            self.lit_pixels_in_frame.contains(&(row, col))
        }
    }

    fn window_values(&self, row: i32, col: i32) -> [bool; 9] {
        [
            (row - 1, col - 1),
            (row - 1, col),
            (row - 1, col + 1),
            (row, col - 1),
            (row, col),
            (row, col + 1),
            (row + 1, col - 1),
            (row + 1, col),
            (row + 1, col + 1),
        ]
        .map(|(row, col)| self.get_pixel(row, col))
    }

    fn enhance_pixel(&self, alg: Alg, row: i32, col: i32) -> bool {
        fn to_decimal(bitarray: [bool; 9]) -> usize {
            bitarray
                .iter()
                .fold(0, |acc, elem| acc * 2 + (*elem as usize))
        }
        let window = self.window_values(row, col);
        let idx = to_decimal(window);
        *alg.get(idx).unwrap()
    }
}

fn main() -> anyhow::Result<()> {
    let (alg, input) = (include_str!("../../input.txt"))
        .split_once("\n\n")
        .expect("invalid input");

    let alg = parse_alg(alg);
    let mut img = parse_img(input);
    println!("part 1: {}", part1(&mut img, alg));
    println!("part 2: {}", part2(&mut img, alg));
    Ok(())
}

fn part1(img: &mut LazyImage, alg: Alg) -> usize {
    img.enhance(alg);
    img.enhance(alg);
    img.lit_pixels_in_frame.len()
}

fn part2(img: &mut LazyImage, alg: Alg) -> usize {
    for _ in 0..48 {
        img.enhance(alg);
    }
    img.lit_pixels_in_frame.len()
}

fn parse_alg(s: &str) -> BitArray<LocalBits, [usize; 8]> {
    let mut result = bitarr![0; 512];
    s.lines()
        .flat_map(|line| line.bytes())
        .enumerate()
        .for_each(|(i, val)| {
            if val == b'#' {
                result.set(i, true);
            }
        });
    result
}

fn parse_img(s: &str) -> LazyImage {
    let mut lit_pixels = HashSet::new();
    let lines = s.lines().map(|line| line.as_bytes()).collect_vec();
    for (r, row) in lines.iter().enumerate() {
        for c in 0..lines[r].len() {
            if row[c] == b'#' {
                lit_pixels.insert((r as i32, c as i32));
            }
        }
    }
    LazyImage {
        lit_pixels_in_frame: lit_pixels,
        frame: Frame {
            top_row: 0,
            bottom_row: lines.len() as i32 - 1,
            left_col: 0,
            right_col: lines[0].len() as i32 - 1,
        },
        lit_outside_frame: false,
    }
}

fn _parse_img(s: &str) -> Vec<BitVec> {
    s.lines()
        .map(|line| {
            let line = line.as_bytes();
            let mut result = bitvec![0; line.len()];
            for (i, c) in line.iter().enumerate() {
                if *c == b'#' {
                    result.set(i, true);
                }
            }
            result
        })
        .collect_vec()
}
