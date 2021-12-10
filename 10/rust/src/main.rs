enum Analysis {
    Corrupted(char),
    Incomplete(Vec<char>),
    Complete,
}

fn main() {
    let analyzed_chunks: Vec<Analysis> = include_str!("../../input.txt")
        .lines()
        .map(analyze_chunk)
        .collect();
    println!("part 1: {}", part1(&analyzed_chunks));
    println!("part 2: {}", part2(&analyzed_chunks));
}

fn part1(analyzed_chunks: &[Analysis]) -> i64 {
    analyzed_chunks
        .iter()
        .map(|chunk| match chunk {
            Analysis::Corrupted(c) => error_score(*c),
            _ => 0,
        })
        .sum()
}

fn part2(analyzed_chunks: &[Analysis]) -> i64 {
    let mut scores: Vec<i64> = analyzed_chunks
        .iter()
        .filter_map(|analysis| match analysis {
            Analysis::Incomplete(stack) => Some(stack),
            _ => None,
        })
        .map(|stack| stack.iter().rev().fold(0, |acc, &c| acc * 5 + score(c)))
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn analyze_chunk(chunk: &str) -> Analysis {
    let mut stack = Vec::new();
    for c in chunk.chars() {
        match get_matching(c) {
            Some(open) => match stack.last() {
                Some(&ch) if ch == open => {
                    stack.pop();
                }
                _ => return Analysis::Corrupted(c),
            },
            None => {
                stack.push(c);
            }
        };
    }
    if stack.is_empty() {
        Analysis::Complete
    } else {
        Analysis::Incomplete(stack)
    }
}

fn get_matching(c: char) -> Option<char> {
    match c {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        '>' => Some('<'),
        _ => None,
    }
}

fn error_score(c: char) -> i64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0,
    }
}

fn score(c: char) -> i64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}
