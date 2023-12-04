def read_file():
    result = []
    with open("./input.txt") as f:
        for line in f:
            result.append(line.strip())
    return result


def find_digit(line):
    for i, c in enumerate(line):
        if c.isdigit():
            return c


def calculate(line):
    first_digit = find_digit(line)
    last_digit = find_digit(line[::-1])
    return first_digit + last_digit


def main():
    result = 0
    file = read_file()
    for line in file:
        result += int(calculate(line))
    print(result)


if __name__ == "__main__":
    main()
