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
    all_seeds = [int(x) for x in file[0][0].split(": ")[1].split(" ")]
    seeds = []
    for i in range(0,len(all_seeds),2):
        seeds.append([all_seeds[i], all_seeds[i+1]])

    maps = []
    for map in file[1:]:
        sub_map=[]
        for map_line in map[1:]:
            tmp = [int(x) for x in map_line.split(" ")]
            sub_map.append(tmp)
        sub_map.sort(key=lambda x: x[1])
        maps.append(sub_map)
    return seeds, maps[::-1] 


def main():
    seeds, maps = parse_file()
    for i in range(0, 100000000, 1):
        current_value = i
        for map in maps:
            for sub_map in map:
                if current_value >= sub_map[0] and current_value < sub_map[0]+sub_map[2]:
                    current_value = current_value - sub_map[0] + sub_map[1]
                    break
        for seed in seeds:
            if current_value >= seed[0] and current_value < seed[0]+seed[1]:
                print(i)
                return

    
if __name__ == "__main__":
    main()
