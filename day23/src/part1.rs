use std::{collections::HashSet, fs};

type Point = (usize, usize);

fn dfs(
    maze: &Vec<&str>,
    visited: &mut HashSet<Point>,
    current: Point,
    end: Point,
    path: &mut Vec<Point>,
) -> Vec<Point> {
    if current == end {
        // Reached the exit, return the path
        path.clone()
    } else {
        visited.insert(current);

        // Define possible moves (up, down, left, right)
        let moves = [(-1, 0), (0, 1), (1, 0), (0, -1)];

        let mut longest_path = Vec::new();

        let mut dir = 0;
        for &(dx, dy) in moves.iter() {
            let new_x = current.0 as i32 + dx;
            let new_y = current.1 as i32 + dy;

            // Check if the new position is within bounds
            if new_x >= 0 && new_x < maze.len() as i32 && new_y >= 0 && new_y < maze[0].len() as i32
            {
                let new_point = (new_x as usize, new_y as usize);

                // Check if the new position is a valid path and has not been visited
                if (maze[new_point.0].chars().nth(new_point.1).unwrap() == '.'
                    || (maze[new_point.0].chars().nth(new_point.1).unwrap() == '^' && dir == 0)
                    || (maze[new_point.0].chars().nth(new_point.1).unwrap() == '>' && dir == 1)
                    || (maze[new_point.0].chars().nth(new_point.1).unwrap() == 'v' && dir == 2)
                    || (maze[new_point.0].chars().nth(new_point.1).unwrap() == '<' && dir == 3))
                    && !visited.contains(&new_point)
                {
                    path.push(new_point.clone());
                    let result = dfs(maze, visited, new_point, end, path);
                    if result.len() > longest_path.len() {
                        longest_path = result;
                    }
                    path.pop(); // Backtrack inside the loop
                }
            }
            dir += 1;
        }

        visited.remove(&current); // Remove current point from visited when backtracking
        longest_path
    }
}

fn find_longest_path(maze: &Vec<&str>, start: Point, end: Point) -> Vec<Point> {
    let mut visited = HashSet::new();
    let mut path = vec![start];
    dfs(maze, &mut visited, start, end, &mut path)
}

fn main() {
    let contents: String = fs::read_to_string("./src/input.txt").expect("File not found");
    let original_input: Vec<&str> = contents.lines().collect();

    let mut start = (0, 0);
    let mut end = (4, 2);

    match original_input[0].find('.') {
        Some(index) => start = (0, index),
        None => println!("Character '.' not found"),
    }

    match original_input[original_input.len() - 1].find('.') {
        Some(index) => end = (original_input.len() - 1, index),
        None => println!("Character '.' not found"),
    }

    let longest_path = find_longest_path(&original_input, start, end);

    if longest_path.is_empty() {
        println!("No path from start to exit exists.");
    } else {
        // let mut index = 0;
        // for line in original_input {
        //     for (i, c) in line.chars().enumerate() {
        //         if longest_path.contains(&(index, i)) {
        //             print!("O")
        //         } else {
        //             print!("{}", c)
        //         }
        //     }
        //     println!();
        //     index += 1
        // }
        println!(
            "Longest path from start to exit! Length: {}",
            longest_path.len() - 1
        );
    }
}
