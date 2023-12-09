use std::collections::HashMap;
use std::fs;

const CARDS: [&str; 13] = [
    "J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A",
];

struct Hand {
    strength: usize,
    cards: String,
    bid: i32,
}

fn count_characters(input: &str) -> Vec<usize> {
    let mut char_count: HashMap<char, usize> = HashMap::new();

    for ch in input.chars() {
        let counter = char_count.entry(ch).or_insert(0);
        *counter += 1;
    }

    let joker_count_alias = char_count.entry('J').or_insert(0);
    let joker_count = joker_count_alias.clone();
    *joker_count_alias = 0;

    let mut counts: Vec<usize> = char_count.values().cloned().collect();
    counts.sort();
    counts.reverse();
    counts[0] += joker_count;

    counts
}

fn measure_strength(input: Vec<usize>) -> usize {
    if input[0] == 2 && input[1] == 2 {
        3
    } else if input[0] == 3 && input[1] == 2 {
        5
    } else if input[0] == 3 {
        4
    } else if input[0] >= 4 {
        input[0] + 2
    } else {
        input[0]
    }
}

fn main() {
    let contents: String = fs::read_to_string("./src/input.txt").expect("Files to be found");
    let original_input: Vec<&str> = contents.lines().collect();
    let mut hands: Vec<Hand> = Vec::with_capacity(original_input.len());

    for i in 0..original_input.len() {
        let parsed_input: Vec<String> = original_input[i]
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let bid: i32 = parsed_input[1].parse::<i32>().unwrap();
        let counts: Vec<usize> = count_characters(&parsed_input[0]);
        let strength: usize = measure_strength(counts.clone());
        hands.push(Hand {
            strength,
            cards: parsed_input[0].clone(),
            bid,
        });
    }

    hands.sort_by(|a, b| {
        a.strength.cmp(&b.strength).then_with(|| {
            let a_indices: Vec<usize> = a
                .cards
                .chars()
                .map(|c| CARDS.iter().position(|&x| x == c.to_string()).unwrap())
                .collect();
            let b_indices: Vec<usize> = b
                .cards
                .chars()
                .map(|c| CARDS.iter().position(|&x| x == c.to_string()).unwrap())
                .collect();
            a_indices.cmp(&b_indices)
        })
    });

    let mut result: i32 = 0;
    for i in 0..hands.len() {
        println!(
            "Cards {:?}, Bid: {:?}, Rank: {:?}",
            hands[i].cards,
            hands[i].bid,
            i + 1
        );
        result += hands[i].bid * (i as i32 + 1)
    }
    println!("result {:?}", result);
}
