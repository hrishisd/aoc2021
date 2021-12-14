def part1(dots, first_fold):
    axis, position = first_fold
    return len(fold(dots, axis, position))


def part2(dots, folds):
    def display():
        width = max(x for x, _ in dots) + 1
        height = max(y for _, y in dots) + 1

        grid = [["."] * width for _ in range(height)]
        for x, y in dots:
            grid[y][x] = "#"
        for row in grid:
            print("".join(row))

    for axis, position in folds:
        dots = fold(dots, axis, position)
    display()


def fold(dots, axis, position):
    def flip(dot):
        x, y = dot
        if axis == "x" and x > position:
            return position - (x - position), y
        elif axis == "y" and y > position:
            return x, position - (y - position)
        else:
            return dot

    return {flip(dot) for dot in dots}


with open("../input.txt") as f:
    dots_str, folds_str = f.read().split("\n\n")
    split_dots = (line.split(",") for line in dots_str.splitlines())
    dots = [(int(split[0]), int(split[1])) for split in split_dots]
    split_folds = (line[11:].split("=") for line in folds_str.splitlines())
    folds = [(axis, int(position)) for axis, position in split_folds]

    print(part1(dots, folds[0]))
    part2(dots, folds)
