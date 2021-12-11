import itertools
import copy


def part1(grid):
    result = 0
    for i in range(100):
        result += step(grid)
    return result


def part2(grid):
    for i in itertools.count(start=1):
        if step(grid) == 100:
            return i


def step(grid):
    def neighbors(x, y):
        def in_range(x, y):
            return x >= 0 and y >= 0 and x < 10 and y < 10

        deltas = itertools.product((-1, 0, 1), repeat=2)
        return [
            (x + dx, y + dy)
            for dx, dy in deltas
            if in_range(x + dx, y + dy) and (dx, dy) != (0, 0)
        ]

    def single_pass(grid):
        flashes = 0
        for x in range(10):
            for y in range(10):
                if grid[x][y] > 9:
                    flashes += 1
                    grid[x][y] = 0
                    for x_, y_ in neighbors(x, y):
                        if grid[x_][y_] > 0:
                            grid[x_][y_] += 1
        return flashes

    for row in grid:
        for i in range(10):
            row[i] += 1

    total_flashes = 0
    while True:
        flashes = single_pass(grid)
        if flashes == 0:
            return total_flashes
        total_flashes += flashes


with open("../input.txt") as f:
    grid = [[int(c) for c in line.strip()] for line in f]
    print(part1(copy.deepcopy(grid)))
    print(part2(grid))
