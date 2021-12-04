from collections import Counter


def to_decimal(bitstring):
    with_place = enumerate(reversed(bitstring))
    return sum(bit * (2 ** place) for place, bit in with_place)


def part1(bitstrings):
    def most_common_bit(bits):
        counts = Counter(bits)
        return 1 if counts[1] >= counts[0] else 0

    def complement(bitstring):
        return [1 - bit for bit in bitstring]

    gamma = [most_common_bit(bits) for bits in zip(*bitstrings)]
    epsilon = complement(gamma)
    return to_decimal(gamma) * to_decimal(epsilon)


def part2(bitstrings):
    def split_strings_by_ith_bit(bitstrings, i):
        zeros, ones = [], []
        for bits in bitstrings:
            (zeros if bits[i] == 0 else ones).append(bits)
        return zeros, ones

    def oxygen_generator(candidates, i=0):
        if len(candidates) == 1:
            return candidates[0]
        zeros, ones = split_strings_by_ith_bit(candidates, i)
        return oxygen_generator(max(ones, zeros, key=len), i + 1)

    def co2_scrubber(candidates, i=0):
        if len(candidates) == 1:
            return candidates[0]
        zeros, ones = split_strings_by_ith_bit(candidates, i)
        return co2_scrubber(min(zeros, ones, key=len), i + 1)

    return to_decimal(oxygen_generator(bitstrings)) * to_decimal(
        co2_scrubber(bitstrings)
    )


with open("../input.txt") as f:
    bitstrings = [[int(b) for b in line.strip()] for line in f]
    print(part1(bitstrings))
    print(part2(bitstrings))
