use regex::Regex;
use std::collections::HashMap;
use std::{fs, vec};

struct Rule {
    property: String,
    sign: String,
    value: i32,
    target: String,
}

fn split_beam(rule: &Rule, beam: &Vec<[i32; 2]>) -> [Vec<[i32; 2]>; 2] {
    let property_to_check = match rule.property.as_str() {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => return [beam.to_vec(), vec![]],
    };

    let mut split_beam = [beam.to_vec(), beam.to_vec()];
    match rule.sign.as_str() {
        ">" => {
            split_beam[0][property_to_check][0] = rule.value + 1;
            split_beam[1][property_to_check][1] = rule.value;
        }
        "<" => {
            split_beam[0][property_to_check][1] = rule.value - 1;
            split_beam[1][property_to_check][0] = rule.value;
        }
        _ => {}
    }

    split_beam
}

fn explore_rules(
    rules_map: &HashMap<String, Vec<Rule>>,
    next_rule: &String,
    beam: &Vec<[i32; 2]>,
) -> Vec<Vec<[i32; 2]>> {
    if next_rule == "A" {
        return vec![beam.to_vec()];
    };
    if next_rule == "R" {
        return vec![];
    };

    let mut all_accepted_beams: Vec<Vec<[i32; 2]>> = vec![];
    let mut curr_beam = beam.to_owned();
    match rules_map.get(next_rule) {
        Some(rules) => {
            for rule in &rules[..rules.len()] {
                let new_beams = split_beam(rule, &curr_beam);
                all_accepted_beams.append(&mut explore_rules(
                    rules_map,
                    &rule.target,
                    &new_beams[0],
                ));
                curr_beam = new_beams[1].to_owned();
            }
        }
        _ => {}
    }
    all_accepted_beams
}

fn main() {
    let contents: String = fs::read_to_string("./src/input.txt").expect("File not found");
    let original_input: Vec<&str> = contents.lines().collect();

    // let mut parts: Vec<Part>;
    let re1 = Regex::new(r"[<>=]").unwrap();
    let re2 = Regex::new(r"[:]").unwrap();
    let mut first_part = true;
    let mut rules_map = HashMap::new();
    for i in 0..original_input.len() {
        if original_input[i].len() == 0 {
            first_part = false;
            continue;
        }
        if first_part {
            match original_input[i].find('{') {
                Some(index) => {
                    let rule = original_input[i][..index].to_owned();
                    let next_rules: Vec<&str> = original_input[i]
                        [index + 1..original_input[i].len() - 1]
                        .split(",")
                        .collect();

                    let mut parsed_rules: Vec<Rule> = vec![];
                    for j in 0..next_rules.len() - 1 {
                        let next_rule = next_rules[j];
                        let sign1 = re1.find(next_rule).unwrap();
                        let sign2: regex::Match<'_> = re2.find(next_rule).unwrap();
                        parsed_rules.push(Rule {
                            property: next_rule[..sign1.start()].to_string(),
                            sign: next_rule[sign1.start()..sign1.end()].to_string(),
                            value: next_rule[sign1.end()..sign2.start()]
                                .parse::<i32>()
                                .unwrap(),
                            target: next_rule[sign2.end()..].to_string(),
                        });
                    }
                    parsed_rules.push(Rule {
                        property: "".to_owned(),
                        sign: "".to_owned(),
                        value: 0,
                        target: next_rules[next_rules.len() - 1].to_string(),
                    });

                    rules_map.insert(rule, parsed_rules);
                }
                None => {}
            }
        } else {
        }
    }

    let beam: Vec<[i32; 2]> = vec![[1, 4000], [1, 4000], [1, 4000], [1, 4000]];
    let accepted_beams = explore_rules(&rules_map, &"in".to_owned(), &beam);

    let result = accepted_beams
        .iter()
        .map(|a| {
            a.iter()
                .map(|x| (x[1] + 1) as usize - x[0] as usize)
                .product::<usize>()
        })
        .sum::<usize>();

    println!("{}", result);
}
