from typing import Tuple
from functools import reduce


def part1(steps):
    def sum_pairs(p1, p2):
        return (p1[0] + p2[0], p1[1] + p2[1])

    x, y = reduce(sum_pairs, steps)
    return x * y


def part2(steps):
    aim, x_pos, depth = 0, 0, 0
    for fwd, aim_delta in steps:
        if aim_delta:
            aim += aim_delta
        else:
            x_pos += fwd
            depth += aim * fwd
    return x_pos * depth


with open("../input.txt") as f:

    def parse_step(line) -> Tuple[int, int]:
        direction, magnitude = line.split()
        magnitude = int(magnitude)
        if direction == "up":
            return 0, -magnitude
        elif direction == "down":
            return 0, magnitude
        elif direction == "forward":
            return magnitude, 0
        else:
            raise ValueError()

    steps = [parse_step(line.strip()) for line in f]
    print(part1(steps))
    print(part2(steps))
