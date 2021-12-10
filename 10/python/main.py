from functools import reduce
from statistics import median

CORRUPTED, INCOMPLETE, COMPLETE = 1, 2, 3


def part1(processed):
    points = {")": 3, "]": 57, "}": 1197, ">": 25137}
    return sum((points[c] for status, c in processed if status is CORRUPTED))


def part2(processed):
    def score(stack):
        return reduce(lambda acc, c: 5 * acc + points[c], reversed(stack), 0)

    points = {"(": 1, "[": 2, "{": 3, "<": 4}
    return median([score(stack) for status, stack in processed if status is INCOMPLETE])


with open("../input.txt") as f:

    def process(chunk):
        close_to_open = {"]": "[", ")": "(", ">": "<", "}": "{"}
        stack = []
        for c in chunk:
            matching = close_to_open.get(c, None)
            if matching is None:
                stack.append(c)
            elif stack[-1] == matching:
                stack.pop()
            else:
                return (CORRUPTED, c)
        return (INCOMPLETE, stack) if stack else (COMPLETE, None)

    chunks = [line.strip() for line in f]
    processed = [process(chunk) for chunk in chunks]
    print(part1(processed))
    print(part2(processed))
