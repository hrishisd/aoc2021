import math
import heapq


def shortest_path(grid):
    def unvisited_neighbors(row, col):
        def in_range(r, c):
            return r >= 0 and c >= 0 and r < len(grid) and c < len(grid[0])

        return (
            next_coord
            for next_coord in (
                (row + 1, col),
                (row - 1, col),
                (row, col + 1),
                (row, col - 1),
            )
            if next_coord in unvisited and in_range(*next_coord)
        )

    n_rows, n_cols = len(grid), len(grid[0])
    unvisited = {(row, col) for row in range(n_rows) for col in range(n_cols)}
    distances = {(r, c): math.inf for r in range(n_rows) for c in range(n_cols)}
    distances[(0, 0)] = 0
    heap = []
    heapq.heappush(heap, (0, (0, 0)))
    while unvisited:
        dist_to_curr, curr = heapq.heappop(heap)
        if curr not in unvisited:
            continue
        unvisited.remove(curr)
        for r, c in unvisited_neighbors(*curr):
            total_dist = dist_to_curr + grid[r][c]
            if total_dist < distances[(r, c)]:
                distances[(r, c)] = total_dist
                heapq.heappush(heap, (total_dist, (r, c)))

    return distances[(n_rows - 1, n_cols - 1)]


def make_big_grid(grid):
    n_rows, n_cols = len(grid), len(grid[0])
    result = [[0] * 5 * n_cols for _ in range(5 * n_rows)]
    for r in range(len(grid)):
        for c in range(len(grid)):
            for i in range(5):
                for j in range(5):
                    val = (grid[r][c] + i + j - 1) % 9 + 1
                    result[r + i * n_rows][c + j * n_cols] = val
    return result


with open("../input.txt") as f:
    grid = [[int(c) for c in line.strip()] for line in f]
    print(shortest_path(grid))
    print(shortest_path(make_big_grid(grid)))
