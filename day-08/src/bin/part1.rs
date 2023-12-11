use std::{collections::HashMap, ops::Add};

fn main() {
    let input = include_str!("./input-1.txt");
    println!("{}", solution(input));
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

fn solution(input: &str) -> u32 {
    let mut network_map: HashMap<String, Node> = HashMap::new();

    input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .get(2..)
        .expect("The network map has more than 3 lines.")
        .into_iter()
        .for_each(|line| {
            network_map.insert(
                line.replace(" ", "")
                    .split("=")
                    .collect::<Vec<&str>>()
                    .first()
                    .expect("The node key exists.")
                    .to_string(),
                Node {
                    left: line
                        .replace(" ", "")
                        .split("=")
                        .collect::<Vec<&str>>()
                        .last()
                        .expect("The node directions exsit.")
                        .split(",")
                        .collect::<Vec<&str>>()
                        .first()
                        .expect("The left direction exists.")
                        .replace("(", "")
                        .to_string(),
                    right: line
                        .replace(" ", "")
                        .split("=")
                        .collect::<Vec<&str>>()
                        .last()
                        .expect("The node directions exsit.")
                        .split(",")
                        .collect::<Vec<&str>>()
                        .last()
                        .expect("The left direction exists.")
                        .replace(")", "")
                        .to_string(),
                },
            );
        });
    let mut current_node: &str = "AAA";
    let mut directions_iterator = input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .first()
        .expect("The directions exist.")
        .chars()
        .map(|character| character.to_string())
        .into_iter()
        .cycle();
    let mut number_of_steps_taken = 0;
    while let Some(direction) = directions_iterator.next() {
        match current_node {
            "ZZZ" => break,
            _ => {
                let next_node = network_map
                    .get(&String::from(current_node))
                    .expect("The current node exists.");

                current_node = match direction.as_str() {
                    "L" => &next_node.left,
                    "R" => &next_node.right,
                    value => panic!("Invalid direction: {}", value),
                };
                number_of_steps_taken = number_of_steps_taken.add(1);
            }
        }
    }
    number_of_steps_taken
}
