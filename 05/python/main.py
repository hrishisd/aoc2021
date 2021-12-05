def part1(grid, segments):
    populate_grid(grid, filter(is_horizontal_or_vertical, segments))
    return num_overlapping_points(grid)


def part2(grid, segments):
    populate_grid(
        grid, (s for s in segments if not is_horizontal_or_vertical(s)))
    return num_overlapping_points(grid)


def num_overlapping_points(grid):
    return len([sq for row in grid for sq in row if sq == 2])


def is_horizontal_or_vertical(segment):
    (x1, y1), (x2, y2) = segment
    return x1 == x2 or y1 == y2


def populate_grid(grid, segments):
    for (x1, y1), (x2, y2) in segments:
        vertical_diff = y2 - y1
        horizontal_diff = x2 - x1
        max_diff = max(abs(vertical_diff), abs(horizontal_diff))
        vertical_step = vertical_diff // abs(max_diff)
        horizontal_step = horizontal_diff // abs(max_diff)
        while x1 != x2 or y1 != y2:
            grid[x1][y1] = 1 if grid[x1][y1] == 0 else 2
            x1 += horizontal_step
            y1 += vertical_step
        grid[x1][y1] = 1 if grid[x1][y1] == 0 else 2


with open("../input.txt") as f:

    def parse_segment(line):
        p1, _, p2 = line.strip().split()
        x1, y1 = p1.split(",")
        x2, y2 = p2.split(",")
        return (int(x1), int(y1)), (int(x2), int(y2))

    segments = [parse_segment(line) for line in f]
    grid = [[0] * 1000 for _ in range(1000)]
    print("part 1", part1(grid, segments))
    print("part 2", part2(grid, segments))
