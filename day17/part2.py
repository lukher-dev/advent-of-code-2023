from heapq import heapify, heappop, heappush


def read_lines():
    with open("./input.txt") as f:
        result = f.read().splitlines()
        return result


def get_neighbors(node):
    return (
        (node[0] + 1, node[1]),
        (node[0], node[1] + 1),
        (node[0] - 1, node[1]),
        (node[0], node[1] - 1),
    )


def find_end(grid):
    end = (len(grid[0]) - 1, len(grid) - 1)
    heapify(nodes_queue := [(0, 1, 1, (0, 0))])
    distances = {}
    while nodes_queue:
        distance, prev_dir_count, prev_dir, curr_node = heappop(nodes_queue)

        if (key := (curr_node, prev_dir, prev_dir_count)) in distances and distances[
            key
        ] <= distance:
            continue

        if curr_node == end and prev_dir_count >= 4:
            return distance

        distances[key] = distance
        neighbours = get_neighbors(curr_node)
        for dir in [(prev_dir - 1) % 4, prev_dir, (prev_dir + 1) % 4]:
            same_dir = dir == prev_dir
            if (not same_dir and prev_dir_count < 4) or (
                same_dir and prev_dir_count >= 10
            ):
                continue

            dir_count = prev_dir_count + 1 if same_dir else 1
            neighbour = neighbours[dir]
            if (
                neighbour[0] < len(grid[0])
                and neighbour[1] < len(grid)
                and neighbour[0] >= 0
                and neighbour[1] >= 0
                and distances.get((neighbour, dir, dir_count), distance + 1) > distance
            ):
                heappush(
                    nodes_queue,
                    (
                        distance + int(grid[neighbour[1]][neighbour[0]]),
                        dir_count,
                        dir,
                        neighbour,
                    ),
                )


def main():
    input = read_lines()
    result = find_end(input)
    print(result)


if __name__ == "__main__":
    main()
