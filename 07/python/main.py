import statistics

with open("../input.txt") as f:

    def part1():
        median = int(statistics.median(xs))
        return sum((abs(median - x) for x in xs))

    def part2():
        def cost(distance):
            return sum(range(distance + 1))

        mean = int(statistics.mean(xs))
        return sum((cost(abs(x - mean)) for x in xs))

    xs = [int(s) for s in f.readline().split(",")]
    print(part1())
    print(part2())
