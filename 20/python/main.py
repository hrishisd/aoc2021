import itertools
import math
from functools import reduce


def transform(lit_pixels_in_frame, default_pixel_outside_frame, frame, alg):
    (top_row, bottom_row), (left_col, right_col) = frame

    def window_coordinates(r, c):
        deltas = itertools.product((-1, 0, 1), repeat=2)
        return ((r + dr, c + dc) for dr, dc in deltas)

    def to_decimal(bits):
        return reduce(lambda acc, b: acc * 2 + int(b), bits)

    def transformed_pixel(r, c):
        def in_frame(r, c):
            return r >= top_row and r <= bottom_row and c >= left_col and c <= right_col

        def get_pixel(r, c):
            return (
                (r, c) in lit_pixels_in_frame
                if in_frame(r, c)
                else default_pixel_outside_frame
            )

        window = window_coordinates(r, c)
        bitstring = [get_pixel(r, c) for r, c in window]
        idx = to_decimal(bitstring)
        return alg[idx]

    result = set()
    for row in range(top_row - 2, bottom_row + 2 + 1):
        for col in range(left_col - 2, right_col + 2 + 1):
            if transformed_pixel(row, col):
                result.add((row, col))

    new_default_pixel = transformed_pixel(math.inf, math.inf)

    return result, new_default_pixel


def enhance(lit_pixels, alg, num_rows, num_cols, num_iter):
    default_outside_frame = False
    for i in range(num_iter):
        offset = 2 * i
        frame = (-offset, num_rows + offset), (-offset, num_cols + offset)
        lit_pixels, default_outside_frame = transform(
            lit_pixels, default_outside_frame, frame, alg
        )
    return len(lit_pixels)


def part1(lit_pixels, alg, num_rows, num_cols):
    return enhance(lit_pixels, alg, num_rows, num_cols, 2)


def part2(lit_pixels, alg, num_rows, num_cols):
    return enhance(lit_pixels, alg, num_rows, num_cols, 50)


with open("../input.txt") as f:
    s = f.read().strip()
    alg, img = s.split("\n\n")
    alg = "".join(alg.split())
    alg = [c == "#" for c in alg]
    img = img.split()
    num_rows, num_cols = len(img), len(img[0])
    lit_pixels = {
        (r, c) for r in range(len(img)) for c in range(len(img[r])) if img[r][c] == "#"
    }
    print(part1(lit_pixels, alg, num_rows, num_cols))
    print(part2(lit_pixels, alg, num_rows, num_cols))
