def read_lines():
    result = []
    with open("./input.txt") as f:
        for line in f:
            result.append([int(numeric_string) for numeric_string in line.replace('\n', "").split(" ")])
    return result

def explore_solutions(array):
    result = []
    are_all_the_same = True
    for i in range(len(array)-1):
        result.append(array[i+1]-array[i])
        if result[0] != array[i+1]-array[i]:
            are_all_the_same = False

    if are_all_the_same:
        return result[-1] + array[-1]
    else:
        return explore_solutions(result) + array[-1]


def main():
    lines = read_lines()
    results = 0
    for data_set in lines:
        results += explore_solutions(data_set[::-1])
    print(results)

if __name__ == "__main__":
    main()
