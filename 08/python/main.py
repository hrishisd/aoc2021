from typing import List, Optional
import functools
import itertools

segments_to_digit = {
    frozenset("abcefg"): 0,
    frozenset("cf"): 1,
    frozenset("acdeg"): 2,
    frozenset("acdfg"): 3,
    frozenset("bcdf"): 4,
    frozenset("abdfg"): 5,
    frozenset("abdefg"): 6,
    frozenset("acf"): 7,
    frozenset("abcdefg"): 8,
    frozenset("abcdfg"): 9,
}
digits = frozenset(range(10))
segment_names = ["a", "b", "c", "d", "e", "f", "g"]


def part1(entries):
    return sum(len(val) in [2, 3, 4, 7] for _, rhs in entries for val in rhs)


def part2(entries):
    def solve(signal_patterns, outputs):
        def decode(pattern, assignment) -> Optional[int]:
            segments = frozenset((assignment[c] for c in pattern))
            return segments_to_digit.get(segments)

        def is_valid(assignment):
            decoded_nums = {decode(pattern, assignment) for pattern in signal_patterns}
            return decoded_nums == digits

        def construct_num(digits: List[int]) -> int:
            return functools.reduce(lambda acc, d: acc * 10 + d, digits)

        assignments = (
            dict(zip(segment_names, ordering))
            for ordering in itertools.permutations(segment_names)
        )
        valid_assignment = next(filter(is_valid, assignments))
        output_digits = (decode(pattern, valid_assignment) for pattern in outputs)
        return construct_num(output_digits)

    return sum(solve(*entry) for entry in entries)


with open("../input.txt") as f:
    splits = [line.split("|") for line in f]
    entries = [(lhs.strip().split(), rhs.strip().split()) for lhs, rhs in splits]
    print(part1(entries))
    print(part2(entries))
