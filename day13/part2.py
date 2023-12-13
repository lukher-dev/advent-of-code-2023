def read_lines():
    result = []
    with open("./input.txt") as f:
        temp = []
        for line in f:
            if len(line) == 1:
                result.append(temp)
                temp=[]
                continue
            temp.append(line.replace("\n",""))
        result.append(temp)
    return result

def diff_letters(a,b):
    return sum ( a[i] != b[i] for i in range(len(a)) )

def explore_solutions(data):
    for i in range(len(data)-1):
        if data[i] == data[i+1] or diff_letters(data[i],data[i+1])==1:
            is_mirrored = True
            has_one_deffect = False
            for j in range(min(len(data)-1-i, i+1)):
                if data[i-j] != data[i+1+j]:
                    if has_one_deffect == False and diff_letters(data[i-j],data[i+1+j])==1:
                        has_one_deffect = True
                    else:
                        is_mirrored = False
            if is_mirrored and has_one_deffect:
                print("mirrored: ", i+1)
                return i+1
    return 0

def main():
    input = read_lines()
    print(input)
    results = 0
    for data_set in input:
        results += 100*explore_solutions(data_set)
        results += explore_solutions([list(row) for row in zip(*data_set)])
    print(results)

if __name__ == "__main__":
    main()
