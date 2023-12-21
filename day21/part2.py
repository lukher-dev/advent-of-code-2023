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
        for neighbour in get_neighbors(node):
            if grid[neighbour[0] % len(grid)][neighbour[1] % len(grid[0])] != "#":
                next_nodes.add(neighbour)
    return next_nodes


def main():
    input, start_location = read_lines()

    values = []
    nodes = {start_location}
    for i in range(65):
        nodes = get_nodes(input, nodes)
        print(i, len(nodes))

    print(len(nodes))
    values.append(len(nodes))

    for i in range(131):
        nodes = get_nodes(input, nodes)
        print(i, len(nodes))

    print(len(nodes))
    values.append(len(nodes))

    for i in range(131):
        nodes = get_nodes(input, nodes)
        print(i, len(nodes))

    print(len(nodes))
    values.append(len(nodes))

    target = (26_501_365 - 65) / 131
    a = values[0] / 2 - values[1] + values[2] / 2
    b = -3 * (values[0] / 2) + 2 * values[1] - values[2] / 2
    c = values[0]
    
    print(a * target * target + b * target + c)


if __name__ == "__main__":
    main()
