fn main() {
    let input = include_str!("./input-2.txt");
    println!("{}", solution(input));
}

#[derive(Debug)]
struct Node {
    position: String,
    left: String,
    right: String,
}

fn solution(input: &str) -> usize {
    let network_map = input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .get(2..)
        .expect("The network map has more than 3 lines.")
        .into_iter()
        .map(|line| Node {
            position: line
                .replace(" ", "")
                .split("=")
                .collect::<Vec<&str>>()
                .first()
                .expect("The node key exists.")
                .to_string(),
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
                .replace("(", ""),
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
                .replace(")", ""),
        })
        .collect::<Vec<Node>>();
    let directions = input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .first()
        .expect("The directions exist.")
        .chars()
        .map(|character| character.to_string())
        .collect::<Vec<String>>();
    let number_of_steps_taken = network_map
        .iter()
        .filter(|node| node.position.ends_with("A"))
        .map(|starting_node| {
            let mut current_node = starting_node;
            directions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, direction)| {
                    let current_node_directions: &Node = network_map
                        .iter()
                        .filter(|node| node.position == current_node.position)
                        .collect::<Vec<&Node>>()
                        .first()
                        .expect("The current node exists.");

                    let next_node_position = match direction.as_str() {
                        "L" => <String as From<&String>>::from(&current_node_directions.left),
                        "R" => <String as From<&String>>::from(&current_node_directions.right),
                        value => panic!("Invalid direction: {}", value),
                    };

                    match current_node.position.ends_with("Z") {
                        true => Some(index),
                        false => {
                            current_node = network_map
                                .iter()
                                .filter(|node| node.position == next_node_position)
                                .collect::<Vec<&Node>>()
                                .first()
                                .expect("The next node exists.");
                            None
                        }
                    }
                })
                .expect("The number of steps taken should exist.")
        })
        .collect::<Vec<usize>>();
    println!("{:?}", number_of_steps_taken);
    least_common_multiple(number_of_steps_taken.as_slice())
}

fn least_common_multiple(common_multiples: &[usize]) -> usize {
    if common_multiples.len() == 1 {
        return common_multiples[0];
    }
    let a = common_multiples[0];
    let b = least_common_multiple(&common_multiples[1..]);
    a * b / greatest_common_denominator(a, b)
}

fn greatest_common_denominator(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    greatest_common_denominator(b, a % b)
}
