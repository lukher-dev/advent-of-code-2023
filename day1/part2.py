import re

string_digits = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
]


def read_file():
    result = []
    with open("./input.txt") as f:
        for line in f:
            result.append(line.strip())
    return result


def find_digit(line):
    for i, c in enumerate(line):
        if c.isdigit():
            return [c, i - 1]


def calculate(line):
    first_digit = find_digit(line)
    last_digit = find_digit(line[::-1])
    if last_digit is not None:
        last_digit = [last_digit[0], len(line) - (last_digit[1] + 2)]

    index = 0
    for string in string_digits:
        all_appearances_list = [m.start() for m in re.finditer(string, line)]
        if len(all_appearances_list) > 0:
            if first_digit == None or all_appearances_list[0] < first_digit[1]:
                first_digit = [str(index), all_appearances_list[0]]
            if last_digit == None or all_appearances_list[-1] > last_digit[1]:
                last_digit = [str(index), all_appearances_list[-1]]

        index += 1
    return first_digit[0] + last_digit[0]


def main():
    result = 0
    file = read_file()
    for line in file:
        result += int(calculate(line))
    print(result)


if __name__ == "__main__":
    main()
