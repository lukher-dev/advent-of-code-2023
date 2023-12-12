use std::fs;

fn contains_coordinate(vector: &Vec<(usize, usize)>, which_one: usize, target: usize) -> bool {
    for i in 0..vector.len() {
        if vector[i].0 == target && which_one == 0 {
            return true;
        } else if vector[i].1 == target && which_one == 1 {
            return true;
        }
    }
    false
}

fn main() {
    let contents: String = fs::read_to_string("./src/input.txt").expect("File not found");
    let original_input: Vec<&str> = contents.lines().collect();

    let mut vector: Vec<(usize, usize)> = Vec::new();

    for i in 0..original_input.len() {
        for j in 0..original_input[i].len() {
            if original_input[i].chars().nth(j).unwrap() == '#' {
                vector.push((i, j));
            }
        }
    }

    println!("{:?}", vector);

    let mut empty_columns: Vec<usize> = Vec::new();
    let mut empty_rows: Vec<usize> = Vec::new();
    for i in 0..vector.len() {
        if !contains_coordinate(&vector, 0, i) {
            empty_rows.push(i);
        }
        if !contains_coordinate(&vector, 1, i) {
            empty_columns.push(i);
        }
    }

    println!("empty_rows {:?}", empty_rows);
    println!("empty_columns {:?}", empty_columns);

    for i in 0..empty_rows.len() {
        for j in 0..vector.len() {
            if empty_rows[i] + i * (1000000 - 1) < vector[j].0 {
                vector[j].0 += 1000000 - 1
            }
        }
    }

    for i in 0..empty_columns.len() {
        for j in 0..vector.len() {
            if empty_columns[i] + i * (1000000 - 1) < vector[j].1 {
                vector[j].1 += 1000000 - 1
            }
        }
    }

    println!("{:?}", vector);

    let mut result: i64 = 0;
    for i in 0..vector.len() {
        for j in i..vector.len() {
            result += (vector[i].0 as i64 - vector[j].0 as i64).abs()
                + (vector[i].1 as i64 - vector[j].1 as i64).abs()
        }
    }

    println!("{:?}", result);
}
