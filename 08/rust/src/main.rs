use crate::Segment::{A, B, C, D, E, F, G};
use enum_map::{enum_map, Enum, EnumMap};
use enumset::{enum_set, EnumSet, EnumSetType};
use itertools::Itertools;
use std::collections::HashMap;

#[macro_use]
extern crate lazy_static;

struct Entry {
    signal_patterns: [EnumSet<Segment>; 10],
    output_patterns: [EnumSet<Segment>; 4],
}

#[derive(Debug, Hash, EnumSetType, Enum)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

type Display = EnumSet<Segment>;

lazy_static! {
    static ref DISPLAY_TO_DIGIT: HashMap<Display, usize> = HashMap::from([
        (enum_set!(A | B | C | E | F | G), 0),
        (enum_set!(C | F), 1),
        (enum_set!(A | C | D | E | G), 2),
        (enum_set!(A | C | D | F | G), 3),
        (enum_set!(B | C | D | F), 4),
        (enum_set!(A | B | D | F | G), 5),
        (enum_set!(A | B | D | E | F | G), 6),
        (enum_set!(A | C | F), 7),
        (enum_set!(A | B | C | D | E | F | G), 8),
        (enum_set!(A | B | C | D | F | G), 9),
    ]);
}

const UNIQUE_NUM_SEGMENTS: [usize; 4] = [1, 3, 4, 7];

fn main() -> anyhow::Result<()> {
    let input = include_str!("../../input.txt");
    let entries = input
        .lines()
        .map(parse_entry)
        .collect::<Result<Vec<Entry>, _>>()?;
    println!("part 1: {}", part1(&entries));
    println!("part 2: {}", part2(&entries));
    Ok(())
}

fn part1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .flat_map(|entry| entry.output_patterns)
        .filter(|segments| UNIQUE_NUM_SEGMENTS.contains(&segments.len()))
        .count()
}

fn part2(entries: &[Entry]) -> usize {
    entries.iter().map(solve).sum()
}

fn solve(entry: &Entry) -> usize {
    let segments = [A, B, C, D, E, F, G];
    let assignment = segments
        .into_iter()
        .permutations(7)
        .map(|ordering| {
            let mut result: EnumMap<Segment, Segment> = enum_map! {_ => A};
            segments
                .into_iter()
                .zip(ordering)
                .for_each(|(l, r)| result[l] = r);
            result
        })
        .find(|assignment| valid_assignment(*assignment, entry.signal_patterns))
        .expect("No valid assignments");
    entry
        .output_patterns
        .iter()
        .map(|&display| transform_display(display, assignment))
        .map(read_digit)
        .fold(0, |acc, digit| {
            acc * 10 + digit.expect("invalid rhs display")
        })
}

fn valid_assignment(assignment: EnumMap<Segment, Segment>, signal_patterns: [Display; 10]) -> bool {
    signal_patterns
        .iter()
        .map(|pattern| transform_display(*pattern, assignment))
        .filter_map(read_digit)
        .unique()
        .count()
        == 10
}

fn transform_display(display: Display, mapping: EnumMap<Segment, Segment>) -> Display {
    display.iter().map(|segment| mapping[segment]).collect()
}

fn read_digit(display: Display) -> Option<usize> {
    DISPLAY_TO_DIGIT.get(&display).copied()
}

fn parse_entry(s: &str) -> anyhow::Result<Entry> {
    let (lhs, rhs) = s.trim().split('|').next_tuple().unwrap();
    let lhs: Vec<Display> = lhs
        .trim()
        .split_whitespace()
        .map(parse_display)
        .collect::<anyhow::Result<Vec<_>>>()?;
    let rhs = rhs
        .trim()
        .split_whitespace()
        .map(parse_display)
        .collect::<anyhow::Result<Vec<_>>>()?;
    Ok(Entry {
        signal_patterns: lhs.try_into().expect("wrong lhs size"),
        output_patterns: rhs.try_into().expect("wrong rhs size"),
    })
}

fn parse_display(s: &str) -> anyhow::Result<Display> {
    s.chars()
        .map(|c| match c {
            'a' => Ok(A),
            'b' => Ok(B),
            'c' => Ok(C),
            'd' => Ok(D),
            'e' => Ok(E),
            'f' => Ok(F),
            'g' => Ok(G),
            _ => Err(anyhow::format_err!("invalid character: {}", c)),
        })
        .collect::<anyhow::Result<Display>>()
}
