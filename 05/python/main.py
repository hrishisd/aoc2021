from itertools import repeat


def part1(segments):
    def horizontal(s):
        return s[0][0] == s[1][0] or s[0][1] == s[1][1]

    return solve_num_overlaps(filter(horizontal, segments))


def part2(segments):
    return solve_num_overlaps(segments)


def solve_num_overlaps(segments):
    def sign(n):
        return 0 if n == 0 else 1 if n > 0 else -1

    def inclusive_range(start, stop, step):
        return repeat(start) if step == 0 else range(start, stop + step, step)

    grid = [[0] * 1000 for _ in range(1000)]
    for (x1, y1), (x2, y2) in segments:
        for x, y in zip(
            inclusive_range(x1, x2, sign(x2 - x1)),
            inclusive_range(y1, y2, sign(y2 - y1)),
        ):
            grid[x][y] += 1
    return sum((cell > 1 for row in grid for cell in row))


with open("../input.txt") as f:

    def parse_segment(line):
        p1, _, p2 = line.strip().split()
        x1, y1 = p1.split(",")
        x2, y2 = p2.split(",")
        return (int(x1), int(y1)), (int(x2), int(y2))

    segments = [parse_segment(line) for line in f]
    print("part 1", part1(segments))
    print("part 2", part2(segments))
