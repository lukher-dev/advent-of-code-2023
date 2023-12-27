use std::{
    collections::{HashMap, HashSet},
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point(usize, usize);

struct Graph {
    edges: HashMap<Point, HashMap<Point, usize>>,
    start: Point,
    end: Point,
}

impl Graph {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
            start: Point(0, 0),
            end: Point(0, 0),
        }
    }

    fn add_edge(&mut self, from: Point, to: Point, distance: usize) {
        self.edges
            .entry(from)
            .or_insert_with(HashMap::new)
            .insert(to, distance);

        self.edges
            .entry(to)
            .or_insert_with(HashMap::new)
            .insert(from, distance);
    }

    fn from_maze(maze: &Vec<&str>) -> Self {
        let mut graph = Graph::new();

        for (i, line) in maze.iter().enumerate() {
            for (j, cell) in line.chars().enumerate() {
                let current = Point(i, j);

                // Skip walls
                if cell != '#' {
                    // Check right
                    if j + 1 < maze[i].len() && maze[i].chars().nth(j + 1).unwrap() != '#' {
                        graph.add_edge(current, Point(i, j + 1), 1);
                    }

                    // Check down
                    if i + 1 < maze.len() && maze[i + 1].chars().nth(j).unwrap() != '#' {
                        graph.add_edge(current, Point(i + 1, j), 1);
                    }
                }
            }
        }

        let start = Point(0, maze[0].find('.').unwrap_or(0));
        let end = Point(maze.len() - 1, maze[maze.len() - 1].rfind('.').unwrap_or(0));
        graph.start = start;
        graph.end = end;

        graph
    }

    fn compress(&mut self) {
        loop {
            let mut nodes_to_remove: Vec<Point> = Vec::new();
            let mut nodes_to_collapse: Vec<Point> = Vec::new();
            // Identify nodes with only one neighbor
            for (node, neighbors) in self.edges.iter_mut() {
                if neighbors.len() == 1 && self.start != *node && self.end != *node {
                    nodes_to_remove.push(*node);
                }
                if neighbors.len() == 2 {
                    nodes_to_collapse.push(*node);
                }
            }
            if nodes_to_remove.is_empty() && nodes_to_collapse.is_empty() {
                break;
            }

            for node in nodes_to_collapse {
                let neighbors: Vec<Point> = self.edges[&node].keys().cloned().collect();
                let distance = self.edges[&node][&neighbors[0]] + self.edges[&node][&neighbors[1]];
                self.edges.get_mut(&neighbors[0]).map(|n| {
                    n.remove(&node);
                    n.insert(neighbors[1], distance)
                });
                self.edges.get_mut(&neighbors[1]).map(|n| {
                    n.remove(&node);
                    n.insert(neighbors[0], distance)
                });
                self.edges.remove(&node);
            }
            // Remove identified nodes
            for node in nodes_to_remove {
                if let Some(neighbors) = self.edges.remove(&node) {
                    for (&neighbor, _) in &neighbors {
                        self.edges.get_mut(&neighbor).map(|n| n.remove(&node));
                    }
                }
            }
        }
    }

    fn dfs(
        &self,
        current: Point,
        end: Point,
        mut length: usize,
        current_path: &mut HashSet<Point>,
        mut longest: usize,
    ) -> usize {
        if current == end {
            // Reached the exit, update the longest path if needed
            if length > longest {
                return length;
            } else {
                return longest;
            }
        } else {
            current_path.insert(current);

            if let Some(neighbors) = self.edges.get(&current) {
                let mut valid_moves: Vec<Point> = neighbors
                    .iter()
                    .filter_map(|(&neighbor, &distance)| {
                        if !current_path.contains(&neighbor) {
                            Some(neighbor)
                        } else {
                            None
                        }
                    })
                    .collect();

                // Sort the valid moves by the length of the path
                valid_moves.sort_by_key(|&point| length + self.edges[&current][&point]);

                for new_point in valid_moves {
                    length += self.edges[&current][&new_point];
                    // println!(
                    //     "Current: {:?}, Endpoint: {:?}, New point: {:?}, Length: {}",
                    //     current, end, new_point, length
                    // );

                    longest = self.dfs(new_point, end, length, current_path, longest);

                    length -= self.edges[&current][&new_point];
                }
            }

            current_path.remove(&current); // Remove current point from visited when backtracking
        }
        println!("Longest: {}", longest);
        return longest;
    }

    fn find_longest_path(&self) -> usize {
        let mut current_path = HashSet::new();
        current_path.insert(self.start);
        let longest_path = self.dfs(self.start, self.end, 0, &mut current_path, 0);
        longest_path
    }
}

fn main() {
    let contents: String = fs::read_to_string("./src/input.txt").expect("File not found");
    let original_input: Vec<&str> = contents.lines().collect();

    let mut graph = Graph::from_maze(&original_input);

    println!("Graph (before compression): {:?}", graph.edges);

    graph.compress();

    println!("Graph (after compression): {:?}", graph.edges);

    let longest_path = graph.find_longest_path();

    println!("Longest path from start to exit! Length: {}", longest_path);
}
