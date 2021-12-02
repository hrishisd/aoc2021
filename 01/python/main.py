def solve_part1(depths):
    return sum(prev < curr for prev, curr in zip(depths, depths[1:]))


def solve_part2(depths):
    window_sums = [a+b+c for a, b, c in zip(depths, depths[1:], depths[2:])]
    return solve_part1(window_sums)


with open("../input.txt") as f:
    depths = [int(line) for line in f]
    part1 = solve_part1(depths)
    print(part1)
    part2 = solve_part2(depths)
    print(part2)
