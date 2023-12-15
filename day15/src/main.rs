use array_init::array_init;
use std::fs;

fn hash(input: &str) -> u32 {
    let mut value: u32 = 0;
    for ch in input.chars() {
        value += ch as u32;
        value = value.wrapping_mul(17) % 256;
    }
    value
}

struct Lens {
    label: String,
    lens: i32,
}

fn main() {
    let contents: String = fs::read_to_string("./src/input.txt").expect("File not found");
    let original_input: Vec<&str> = contents.lines().collect();
    let split_input: Vec<&str> = original_input[0].split(",").collect();

    let mut boxes: [Vec<Lens>; 256] = array_init(|_| Vec::new());
    for i in 0..split_input.len() {
        match split_input[i].find('=') {
            Some(index) => {
                let box_number = hash(&split_input[i][..index]);
                let new_lens = Lens {
                    label: split_input[i][..index].to_owned(),
                    lens: split_input[i][index + 1..].parse().unwrap(),
                };
                if let Some(pos) = boxes[box_number as usize]
                    .iter()
                    .position(|x| x.label == split_input[i][..index])
                {
                    boxes[box_number as usize][pos] = new_lens
                } else {
                    boxes[box_number as usize].push(new_lens);
                }
            }
            None => {
                let box_number = hash(&split_input[i][..split_input[i].len() - 1]);
                boxes[box_number as usize]
                    .retain(|x| x.label != split_input[i][..split_input[i].len() - 1]);
            }
        }
        // DEBUG
        // println!("After '{}':", split_input[i]);
        // for (box_number, lenses) in boxes.iter().enumerate() {
        //     if lenses.len() > 0 {
        //         print!("Box {}: ", box_number);
        //         for lens in lenses.iter() {
        //             print!("[{} {}] ", lens.label, lens.lens);
        //         }
        //         println!();
        //     }
        // }
        // println!();
    }

    let mut result = 0;
    for (box_number, lenses) in boxes.iter().enumerate() {
        if lenses.len() > 0 {
            for (slot, lens) in lenses.iter().enumerate() {
                result += (box_number + 1) * (slot + 1) * lens.lens as usize;
            }
        }
    }

    println!("{}", result);
}
