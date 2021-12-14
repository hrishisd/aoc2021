#![feature(array_windows)]

use core::panic;
use itertools::Itertools;

type Rules = [[u8; 26]; 26];
type PairCounts = [[u64; 26]; 26];

fn main() {
    let (template, rules) = parse_input(include_str!("../../input.txt")).expect("invalid input");
    let last_base = template[template.len() - 1];
    let mut pair_counts = create_inital_pair_counts(&template);

    for _ in 0..10 {
        pair_counts = iter(&pair_counts, &rules);
    }
    let part1 = min_max_char_counts(&pair_counts, last_base);
    println!("{}", part1);

    for _ in 0..30 {
        pair_counts = iter(&pair_counts, &rules);
    }
    let part2 = min_max_char_counts(&pair_counts, last_base);
    println!("{}", part2);
}

fn min_max_char_counts(pair_counts: &PairCounts, last_base: u8) -> u64 {
    let mut counts = [0; 26];
    for l in 0..26 {
        for r in 0..26 {
            counts[l] += pair_counts[l][r];
        }
    }
    counts[last_base as usize] += 1;
    match counts.iter().filter(|&&n| n > 0).minmax() {
        itertools::MinMaxResult::MinMax(min, max) => max - min,
        _ => panic!("illegal argument"),
    }
}

fn create_inital_pair_counts(template: &[u8]) -> PairCounts {
    let mut result = [[0; 26]; 26];
    for &[l, r] in template.array_windows() {
        result[l as usize][r as usize] += 1;
    }
    result
}

fn iter(pair_counts: &PairCounts, rules: &Rules) -> PairCounts {
    let mut result = [[0; 26]; 26];
    for l in 0..26 {
        for r in 0..26 {
            let count = pair_counts[l][r];
            if count > 0 {
                let c = rules[l][r] as usize;
                result[l][c] += count;
                result[c][r] += count;
            }
        }
    }
    result
}

fn parse_input(input: &str) -> Option<(Vec<u8>, Rules)> {
    let (template, rule_str) = input.split("\n\n").next_tuple()?;
    let template = template.bytes().map(offset).collect_vec();
    let mut rules = [[0; 26]; 26];
    for line in rule_str.lines() {
        let (lhs, rhs): (&str, &str) = line.split_once(" -> ")?;
        let (l, r) = lhs.bytes().map(offset).next_tuple()?;
        rules[l as usize][r as usize] = offset(rhs.as_bytes()[0]);
    }
    Some((template, rules))
}

fn offset(b: u8) -> u8 {
    b - b'A'
}
