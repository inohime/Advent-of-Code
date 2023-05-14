from enum import Enum

jet_stream = open("input17.txt").read().strip()


class OP(Enum):
    SUB = 0,
    ADD = 1


def new_pattern(idx, y):
    patterns = [
        {(2, y), (3, y), (4, y), (5, y)},
        {(3, y + 2), (2, y + 1), (3, y + 1), (4, y + 1), (3, y)},
        {(2, y), (3, y), (4, y), (4, y + 1), (4, y + 2)},
        {(2, y), (2, y + 1), (2, y + 2), (2, y + 3)},
        {(2, y + 1), (2, y), (3, y + 1), (3, y)}
    ]
    return patterns[idx]


def move_vertical(curr: set, op: OP) -> set:
    return {(x, y - 1 if op == OP.SUB else y + 1) for (x, y) in curr}


def move_horizontal(curr: set, op: OP) -> set:
    if any([(x == 0 and op == OP.SUB) or (x == 6 and op == OP.ADD) for (x, _) in curr]):
        return curr

    return {(x - 1 if op == OP.SUB else x + 1, y) for (x, y) in curr}


def collect(chamber) -> frozenset:
    max_top_bound = max([y for (_, y) in chamber])
    return frozenset([(x, max_top_bound - y) for (x, y) in chamber if max_top_bound - y <= 20])


def simulate(stream: str, MAX_ROCKS: int) -> int:
    add = 0
    i = 0
    top_bound = 0
    idx = 0
    chamber = set()
    cache = {}

    while i < MAX_ROCKS:
        rock = new_pattern(i % 5, top_bound + 4)

        while True:
            if stream[idx] == '>':
                rock = move_horizontal(rock, OP.ADD)
                if rock & chamber:
                    rock = move_horizontal(rock, OP.SUB)
            else:
                rock = move_horizontal(rock, OP.SUB)
                if rock & chamber:
                    rock = move_horizontal(rock, OP.ADD)

            idx = (idx + 1) % len(stream)

            rock = move_vertical(rock, OP.SUB)
            if rock & chamber or any([y == 0 for x, y in rock]):
                rock = move_vertical(rock, OP.ADD)
                chamber |= rock
                top_bound = max([y for (_, y) in chamber])

                if i >= 2022:
                    fc = (i % 5, idx, collect(chamber))
                    if fc in cache:
                        (last_it, last_top_bound) = cache[fc]
                        cl = i - last_it
                        td = top_bound - last_top_bound
                        skip = (MAX_ROCKS - i) // cl
                        add += skip * td
                        i += skip * cl
                    cache[fc] = (i, top_bound)
                break
        i += 1

    return top_bound + add


# part 1
print(simulate(jet_stream, 2022))

# part 2
print(simulate(jet_stream, 1000000000000))
