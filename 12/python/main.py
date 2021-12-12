from collections import defaultdict


def part1(graph):
    return num_paths(graph)


def part2(graph):
    return num_paths(graph, can_visit_twice=True)


def num_paths(graph, can_visit_twice=False):
    def rec(node, graph, visited, can_visit_twice=False):
        match node:
            case "start" if "start" in visited:
                return 0
            case "end":
                return 1
            case _ if node.islower() and not can_visit_twice and node in visited:
                return 0
            case _:
                second_visit = node in visited
                if node.islower():
                    visited.add(node)
                result = 0
                for v in graph[node]:
                    result += rec(
                        v, graph, visited, False if second_visit else can_visit_twice
                    )
                if node.islower() and not second_visit:
                    visited.remove(node)
                return result

    return rec("start", graph, set(), can_visit_twice)


with open("../input.txt") as f:
    edges = [line.strip().split("-") for line in f]
    graph = defaultdict(list)
    for u, v in edges:
        graph[u].append(v)
        graph[v].append(u)

    print(part1(graph))
    print(part2(graph))
