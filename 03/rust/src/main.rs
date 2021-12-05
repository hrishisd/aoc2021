use std::{fmt, num::ParseIntError, str::FromStr};

#[derive(Clone, Copy)]
struct BitString {
    value: u16,
    length: u16,
}

impl BitString {
    fn zeros(length: u16) -> BitString {
        BitString { value: 0, length }
    }

    fn get(&self, i: u16) -> bool {
        (self.value & 1u16 << i) > 0
    }

    fn set(&mut self, i: u16) {
        if i > self.length {
            panic!("Tried to set value out of bitstring range");
        }
        self.value |= 1_u16 << i;
    }

    fn complement(&self) -> BitString {
        let mask = (1 << (self.length - 1)) - 1;
        BitString {
            value: (!self.value) & mask,
            length: self.length,
        }
    }
}

impl fmt::Debug for BitString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Bitstring")
            .field(&format!("{:#b}", self.value))
            .finish()
    }
}

impl FromStr for BitString {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u16::from_str_radix(s, 2).map(|i| BitString {
            value: i,
            length: s.len() as u16,
        })
    }
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../input.txt");
    let bitstrings = input
        .lines()
        .map(BitString::from_str)
        .collect::<Result<Vec<_>, _>>()?;
    println!("part 1: {}", part1(&bitstrings));
    println!("part 2: {}", part2(&bitstrings));
    Ok(())
}

fn part1(bitstrings: &[BitString]) -> i32 {
    let n = bitstrings[0].length;
    let mut gamma = BitString::zeros(n);
    for i in 0..n {
        let set_count = bitstrings.iter().filter(|bits| bits.get(i)).count();
        if set_count > bitstrings.len() / 2 {
            gamma.set(i);
        }
    }
    let epsilon = gamma.complement();
    gamma.value as i32 * epsilon.value as i32
}

fn part2(bitstrings: &[BitString]) -> i32 {
    let n = bitstrings[0].length;
    oxygen_rating(bitstrings, n - 1).value as i32 * co2_rating(bitstrings, n - 1).value as i32
}

fn oxygen_rating(candidates: &[BitString], i: u16) -> BitString {
    filter_down(candidates, i as i16, |zeros, ones| {
        if ones.len() >= zeros.len() {
            ones
        } else {
            zeros
        }
    })
}
fn co2_rating(candidates: &[BitString], i: u16) -> BitString {
    filter_down(candidates, i as i16, |zeros, ones| {
        if zeros.len() <= ones.len() {
            zeros
        } else {
            ones
        }
    })
}

fn filter_down<F>(candidates: &[BitString], i: i16, selector: F) -> BitString
where
    F: Fn(Vec<BitString>, Vec<BitString>) -> Vec<BitString>,
{
    if candidates.len() == 1 {
        return candidates[0];
    }
    let (ones, zeros): (Vec<BitString>, Vec<BitString>) =
        candidates.iter().partition(|bits| bits.get(i as u16));
    filter_down(&selector(zeros, ones), i - 1, selector)
}
