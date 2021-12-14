from collections import defaultdict

with open("../input.txt") as f:

    def iter(pair_counts):
        result = defaultdict(int)
        for pair, count in pair_counts.items():
            c = rules[pair]
            left, right = pair[0] + c, c + pair[1]
            result[left] += count
            result[right] += count
        return result

    def maxmin_diff():
        char_counts = defaultdict(int)
        for pair, count in pair_counts.items():
            char_counts[pair[0]] += count
        char_counts[template[-1]] += 1
        return max(char_counts.values()) - min(char_counts.values())

    template, _, *rules = f.readlines()
    template = template.strip()
    rules = [rule.strip().split(" -> ") for rule in rules]
    rules = {lhs: rhs for lhs, rhs in rules}

    pair_counts = defaultdict(int)
    for a, b in zip(template, template[1:]):
        pair_counts[a + b] += 1

    for _ in range(10):
        pair_counts = iter(pair_counts)
    print(maxmin_diff())
    for _ in range(30):
        pair_counts = iter(pair_counts)
    print(maxmin_diff())
