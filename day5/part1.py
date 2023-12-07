def read_lines():
    result = []
    temp = []
    with open("./input.txt") as f:
        for line in f:
            if len(line) == 1:
                result.append(temp)
                temp = []
                continue
            temp.append(line.strip())
    result.append(temp)
    return result

def parse_file():
    file = read_lines()
    seeds = [int(x) for x in file[0][0].split(": ")[1].split(" ")]
    maps = []
    for map in file[1:]:
        sub_map=[]
        for map_line in map[1:]:
            tmp = [int(x) for x in map_line.split(" ")]
            sub_map.append(tmp)
        sub_map.sort(key=lambda x: x[1])
        maps.append(sub_map)
    return seeds, maps


def main():
    seeds, maps = parse_file()
    result = None
    for seed in seeds:
        current_value = seed
        for map in maps:
            for sub_map in map:
                if current_value >= sub_map[1] and current_value < sub_map[1]+sub_map[2]:
                    current_value = current_value - sub_map[1] + sub_map[0]
                    break
        if result is None or current_value < result:
            result = current_value

    print(result)
if __name__ == "__main__":
    main()
