import math


def neighbors(x, y):
    def in_bounds(x, y):
        return 0 <= x < len(grid) and 0 <= y < len(grid[0])

    return ((x+dx, y+dy)
            for dx, dy in ((0, 1), (0, -1), (1, 0), (-1, 0))
            if in_bounds(x+dx, y+dy))


def part1(grid):
    def is_low_point(x, y):
        return all(grid[x][y] < grid[_x][_y]for _x, _y in neighbors(x, y))

    return sum((
        grid[x][y] + 1
        for x in range(len(grid))
        for y in range(len(grid[0]))
        if is_low_point(x, y)
    ))


def part2(grid):
    def explore(x, y):
        if grid[x][y] == 9:
            return 0
        grid[x][y] = 9
        return 1 + sum(explore(*n) for n in neighbors(x, y))

    basin_sizes = []
    for x in range(len(grid)):
        for y in range(len(grid[x])):
            if grid[x][y] != 9:
                basin_sizes.append(explore(x, y))
    basin_sizes.sort(reverse=True)
    return math.prod(basin_sizes[:3])


with open("../input.txt") as f:
    grid = [[int(c) for c in line.strip()] for line in f]
    # print(grid)
    print(part1(grid))
    print(part2(grid))
