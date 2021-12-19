use itertools::Itertools;
use regex::Regex;
use std::{
    fmt::{Debug, Display},
    num::ParseIntError,
    panic,
    str::FromStr,
};
use Token::*;

#[derive(Clone, Copy, Debug)]
enum Token {
    PairStart,
    PairEnd,
    Sep,
    Num(i32),
}

#[derive(Debug, Clone)]
struct Number(Vec<Token>);

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.0 {
            write!(f, "{}", token)?;
        }
        Ok(())
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PairStart => write!(f, "["),
            Self::PairEnd => write!(f, "]"),
            Self::Sep => write!(f, ","),
            Self::Num(arg0) => write!(f, "{}", arg0),
        }
    }
}

impl FromStr for Token {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "[" => Ok(PairStart),
            "]" => Ok(PairEnd),
            "," => Ok(Sep),
            _ => Ok(Num(s.parse().expect("invalid number"))),
        }
    }
}

impl FromStr for Number {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: use lazy static
        let token_re = Regex::new(r"\d+|[,\[\]]").unwrap();
        let tokens = s
            .split_inclusive(&token_re)
            .map(Token::from_str)
            .collect::<Result<_, _>>()?;
        Ok(Number(tokens))
    }
}

impl Number {
    fn magnitude(&self) -> i64 {
        /// Magnitude of the pair that starts at the slice.
        /// Returns the magnitude and the index of the closing brace of the pair.
        fn rec(tokens: &[Token]) -> (i64, usize) {
            match tokens[0] {
                Num(i) => (i as i64, 0),
                PairStart => {
                    let (mag_l, left_last_idx) = rec(&tokens[1..]);
                    let sep_idx = left_last_idx + 2;
                    assert!(matches!(tokens[sep_idx], Sep), "malformed number");
                    let right_pair_start_idx = sep_idx + 1;
                    let (mag_r, rel_right_last_idx) = rec(&tokens[right_pair_start_idx..]);
                    let pair_end_idx = right_pair_start_idx + rel_right_last_idx + 1;
                    assert!(matches!(tokens[pair_end_idx], PairEnd), "malformed number");
                    let result = 3 * mag_l + 2 * mag_r;
                    (result, pair_end_idx)
                }
                _ => panic!("malformed number"),
            }
        }
        let (result, last_idx) = rec(&self.0);
        assert_eq!(self.0.len() - 1, last_idx, "invalid state");
        result
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    let numbers: Vec<Number> = input
        .lines()
        .map(Number::from_str)
        .collect::<Result<_, _>>()
        .unwrap();
    println!("part 1: {}", part1(&numbers));
    println!("part 2: {}", part2(&numbers));
}

fn part1(numbers: &[Number]) -> i64 {
    let mut result = numbers.first().cloned().unwrap();
    for n in numbers.iter().skip(1) {
        result = sum_and_reduce(&result, n);
    }
    result.magnitude()
}

fn part2(numbers: &[Number]) -> i64 {
    numbers
        .iter()
        .permutations(2)
        .map(|v| sum_and_reduce(v[0], v[1]).magnitude())
        .max()
        .unwrap()
}

fn sum(l: &Number, r: &Number) -> Number {
    let (l, r) = (&l.0, &r.0);
    let mut result = Vec::with_capacity(l.len() + r.len() + 3);
    result.push(PairStart);
    result.extend_from_slice(l);
    result.push(Sep);
    result.extend_from_slice(r);
    result.push(PairEnd);
    Number(result)
}

fn try_explode(n: &mut Number) -> bool {
    let tokens = &mut n.0;
    let mut depth = 0;
    for idx in 0..(tokens.len()) {
        match tokens[idx] {
            PairStart => {
                depth += 1;
                if depth == 5 {
                    // The next 3 tokens should be "l,y".
                    if let [Num(x), Sep, Num(y)] = tokens[idx + 1..(idx + 4)] {
                        // Update the number to the left.
                        for i in (0..idx).rev() {
                            if let Some(Num(i)) = tokens.get_mut(i) {
                                *i += x;
                                break;
                            }
                        }
                        // Update the number to the right.
                        for i in (idx + 6)..tokens.len() {
                            if let Some(Num(i)) = tokens.get_mut(i) {
                                *i += y;
                                break;
                            }
                        }
                        // remove "l,y]"
                        tokens.drain(idx + 1..idx + 5);
                        tokens[idx] = Num(0);
                        return true;
                    } else {
                        panic!("Illegal state");
                    };
                }
            }
            PairEnd => {
                depth -= 1;
            }
            _ => {}
        }
    }
    false
}

fn try_split(n: &mut Number) -> bool {
    let tokens = &mut n.0;
    for idx in 0..tokens.len() {
        match tokens[idx] {
            Num(n) if n >= 10 => {
                let (l, r) = (n / 2, (n + 1) / 2);
                // replace n with "[l,r]"
                let new_pair = [PairStart, Num(l), Sep, Num(r), PairEnd];
                tokens.splice(idx..idx + 1, new_pair);
                return true;
            }
            _ => {}
        }
    }
    false
}

fn reduce(n: &mut Number) {
    loop {
        let exploded = try_explode(n);
        if !exploded {
            let split = try_split(n);
            if !split {
                return;
            }
        }
    }
}

fn sum_and_reduce(n: &Number, m: &Number) -> Number {
    let mut result = sum(n, m);
    reduce(&mut result);
    result
}

#[test]
fn test_parse_round_trip() {
    let cases = [
        "[1,2]",
        "[[1,2],3]",
        "[9,[8,7]]",
        "[[1,9],[8,5]]",
        "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]",
        "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]",
    ];
    for s in cases {
        let n = Number::from_str(s).expect("failed to parse");
        assert_eq!(s, format!("{}", n));
    }
}

#[test]
fn test_add() {
    let l = &"[1,2]".parse().unwrap();
    let r = &"[[3,4],5]".parse().unwrap();
    let result = sum(l, r);
    assert_eq!("[[1,2],[[3,4],5]]", format!("{}", result));
}

#[test]
fn test_explode() {
    let test_cases = [
        ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
        ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
        ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
        (
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        ),
        (
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        ),
    ];

    for (num, expected_result) in test_cases {
        let mut num = num.parse().unwrap();
        try_explode(&mut num);
        assert_eq!(expected_result, format!("{}", num));
    }
}

#[test]
fn test_split() {
    let test_cases = [
        ("[10,2]", "[[5,5],2]"),
        ("[11,2]", "[[5,6],2]"),
        (
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        ),
    ];
    for (num, expected_result) in test_cases {
        let mut num = num.parse().unwrap();
        let did_split = try_split(&mut num);
        assert!(did_split, "failed to do split");
        assert_eq!(expected_result, format!("{}", num));
    }
}

#[test]
fn test_reduce() {
    let n = "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]";
    let expected = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
    let mut n = n.parse().unwrap();
    reduce(&mut n);
    assert_eq!(expected, format!("{}", n));
}

#[test]
fn test_magnitude() {
    let test_cases = [
        ("[[1,2],[[3,4],5]]", 143),
        ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
        ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
        ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
        ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
        (
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            3488,
        ),
    ];
    for (s, expected) in test_cases {
        let n = Number::from_str(s).unwrap();
        assert_eq!(expected, n.magnitude());
    }
}

#[test]
fn test_example() {
    let input = include_str!("../../example.txt");
    let numbers: Vec<Number> = input
        .lines()
        .map(Number::from_str)
        .collect::<Result<_, _>>()
        .unwrap();
    let answer = part1(&numbers);
    assert_eq!(4140, answer);
}

#[test]
fn test_example_part_2() {
    let input = include_str!("../../example.txt");
    let numbers: Vec<Number> = input
        .lines()
        .map(Number::from_str)
        .collect::<Result<_, _>>()
        .unwrap();
    let answer = part2(&numbers);
    assert_eq!(3993, answer);
}
