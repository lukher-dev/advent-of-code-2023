use regex::Regex;
use std::collections::HashMap;
use std::{fs, vec};

struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

struct Rule {
    property: String,
    sign: String,
    value: i32,
    target: String,
}

fn check_rule(rule: &Rule, part: &Part) -> bool {
    let property_to_check = match rule.property.as_str() {
        "x" => part.x,
        "m" => part.m,
        "a" => part.a,
        "s" => part.s,
        _ => 0,
    };

    match rule.sign.as_str() {
        ">" => property_to_check > rule.value,
        "<" => property_to_check < rule.value,
        _ => true,
    }
}

fn check_rules(rules: &Vec<Rule>, part: &Part) -> String {
    for rule in &rules[..rules.len() - 1] {
        if check_rule(rule, part) {
            return rule.target.to_owned();
        }
    }
    return rules[rules.len() - 1].target.to_owned();
}

fn explore_rules(rules_map: &HashMap<String, Vec<Rule>>, next_rule: &String, part: &Part) -> bool {
    println!("{}", next_rule);
    if next_rule == "A" {
        return true;
    };
    if next_rule == "R" {
        return false;
    };

    match rules_map.get(next_rule) {
        Some(rules) => {
            return explore_rules(rules_map, &check_rules(rules, part), part);
        }
        _ => false,
    }
}

fn main() {
    let contents: String = fs::read_to_string("./src/input.txt").expect("File not found");
    let original_input: Vec<&str> = contents.lines().collect();

    // let mut parts: Vec<Part>;
    let re1 = Regex::new(r"[<>=]").unwrap();
    let re2 = Regex::new(r"[:]").unwrap();
    let mut first_part = true;
    let mut rules_map = HashMap::new();
    let mut result = 0;
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
                        println!(
                            "{} {} {} {}",
                            parsed_rules[j].property,
                            parsed_rules[j].sign,
                            parsed_rules[j].value,
                            parsed_rules[j].target
                        );
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
            let parsed_parts: Vec<&str> = original_input[i][1..original_input[i].len() - 1]
                .split(",")
                .collect();
            let part = Part {
                x: parsed_parts[0].split("=").collect::<Vec<&str>>()[1]
                    .parse::<i32>()
                    .unwrap(),
                m: parsed_parts[1].split("=").collect::<Vec<&str>>()[1]
                    .parse::<i32>()
                    .unwrap(),
                a: parsed_parts[2].split("=").collect::<Vec<&str>>()[1]
                    .parse::<i32>()
                    .unwrap(),
                s: parsed_parts[3].split("=").collect::<Vec<&str>>()[1]
                    .parse::<i32>()
                    .unwrap(),
            };

            if explore_rules(&rules_map, &"in".to_owned(), &part) {
                result += part.x + part.m + part.a + part.s;
            }
        }
    }
    println!("{}", result);
}
