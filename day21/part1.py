def read_lines():
    start_position = (0, 0)
    with open("./input.txt") as f:
        result = f.read().splitlines()
        for i in range(len(result)):
            has_s = result[i].find("S")
            if has_s > -1:
                start_position = (i, has_s)
    return result, start_position


def get_neighbors(node):
    return (
        (node[0] - 1, node[1]),
        (node[0], node[1] + 1),
        (node[0] + 1, node[1]),
        (node[0], node[1] - 1),
    )


def get_nodes(grid, nodes):
    next_nodes = set()
    for node in nodes:
        neighbours = get_neighbors(node)
        for neighbour in neighbours:
            if (
                neighbour[0] < len(grid[0])
                and neighbour[1] < len(grid)
                and neighbour[0] >= 0
                and neighbour[1] >= 0
                and grid[neighbour[0]][neighbour[1]] != "#"
            ):
                next_nodes.add(neighbour)
    return next_nodes


def main():
    input, start_location = read_lines()

    nodes = {start_location}
    for i in range(64):
        nodes = get_nodes(input, nodes)

    print(len(nodes))


if __name__ == "__main__":
    main()
