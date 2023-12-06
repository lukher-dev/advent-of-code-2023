use std::fs;

fn complete_number(row: &str, position: usize) -> (i32, usize) {
    let mut number = "".to_owned();
    let mut left = position;
    let mut right = position + 1;

    loop {
        let current_char = row.chars().nth(left).unwrap();
        if current_char.is_digit(10) {
            number = [current_char.to_string(), number].join("");
        } else {
            break;
        }

        if left == 0 {
            break;
        }

        left -= 1;
    }

    while right < row.len() {
        let current_char = row.chars().nth(right).unwrap();
        if current_char.is_digit(10) {
            number = [number, current_char.to_string()].join("");
        } else {
            break;
        }

        if right == row.len() - 1 {
            break;
        }

        right += 1;
    }

    return (number.parse::<i32>().unwrap(), right);
}

fn find_adjacent(map: &Vec<&str>, i: usize, j: usize) -> Vec<i32> {
    let mut numbers: Vec<i32> = vec![];
    for k in 0..3 {
        let mut l = 0;
        while l < 3 {
            if i + k >= 1 && j + l >= 1 && i + k - 1 < map.len() && j + l - 1 < map[i].len() {
                println!("Searching ({}, {})", i + k - 1, j + l - 1);
                if map[i + k - 1].chars().nth(j + l - 1).unwrap().is_digit(10) {
                    let result = complete_number(map[i + k - 1], j + l - 1);
                    l = result.1 - j;
                    numbers.push(result.0)
                }
            }
            l += 1
        }
    }

    return numbers;
}

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Files to be found");
    let original_input: Vec<&str> = contents.lines().collect();
    let mut numbers: Vec<i32> = vec![];
    let mut gears: i32 = 0;
    for i in 0..original_input.len() {
        for j in 0..original_input[i].len() {
            if !(original_input[i].chars().nth(j).unwrap().is_digit(10)
                || original_input[i].chars().nth(j).unwrap() == '.')
            {
                let adjecent = find_adjacent(&original_input, i, j);
                println!("test before: {}", adjecent.len());
                numbers.append(&mut adjecent.clone());

                //part2
                println!("test: {}", adjecent.len());
                if original_input[i].chars().nth(j).unwrap() == '*' && adjecent.len() == 2 {
                    println!("gear");
                    gears += adjecent[0] * adjecent[1];
                }
            }
        }
    }

    let mut sum = 0;
    for number in numbers {
        println!("Number: {}", number);
        sum += number;
    }

    println!("Final sum: {}", sum);

    println!("Final gears: {}", gears);
}
