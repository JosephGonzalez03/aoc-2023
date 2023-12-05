fn main() {
    let input = include_str!("./input-1.txt");
    print!("{}", part1(input));
}

fn part1(input: &str) -> u32 {
    input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .as_slice()
        .windows(3)
        .map(|window| {
            window
                .get(1)
                .expect("Window exists")
                .char_indices()
                .filter(|(_index, character)| character != &'.' && !character.is_digit(10))
                .map(|(index, _character)| {
                    vec![
                        (
                            window.get(1).expect("In window"),
                            vec![index - 1, index + 1],
                        ),
                        (
                            window.first().expect("Window exists"),
                            find_adjacent_number_indices(
                                window.first().expect("Window exists"),
                                index,
                            ),
                        ),
                        (
                            window.last().expect("Window exists"),
                            find_adjacent_number_indices(
                                window.last().expect("Window exists"),
                                index,
                            ),
                        ),
                    ]
                })
                .flatten()
                .map(|(input_line, number_indices)| {
                    number_indices
                        .into_iter()
                        .map(|number_index| find_part_number(input_line, number_index))
                        .collect::<Vec<u32>>()
                })
                .flatten()
                .collect::<Vec<u32>>()
        })
        .flatten()
        .sum()
}

fn find_adjacent_number_indices(input_line: &str, symbol_index: usize) -> Vec<usize> {
    let mut char_iter = input_line
        .char_indices()
        .filter(|character_tuple| {
            vec![symbol_index - 1, symbol_index, symbol_index + 1].contains(&character_tuple.0)
        })
        .peekable();
    let mut indices: Vec<usize> = vec![];

    while let Some((current_index, current_character)) = char_iter.next() {
        match char_iter.peek() {
            Some((_next_index, next_character)) => {
                if current_character.is_digit(10) && next_character == &'.' {
                    indices.push(current_index)
                }
            }
            None => {
                if current_character.is_digit(10) {
                    indices.push(current_index)
                }
            }
        }
    }
    indices
}

fn find_part_number(input_line: &str, number_index: usize) -> u32 {
    let left_index = match input_line
        .match_indices(|character: char| !character.is_digit(10))
        .filter(|match_result| match_result.0 < number_index)
        .map(|match_result| match_result.0)
        .collect::<Vec<usize>>()
    {
        indices if indices.len() > 0 => *indices.last().expect("Should exist") + 1,
        _ => 0,
    };
    let right_index = match input_line
        .match_indices(|character: char| !character.is_digit(10))
        .filter(|match_result| match_result.0 > number_index)
        .map(|match_result| match_result.0)
        .collect::<Vec<usize>>()
    {
        indices if indices.len() > 0 => *indices.first().expect("Should exist"),
        _ => input_line.len(),
    };
    match input_line
        .get(left_index..right_index)
        .expect("Bound checked")
        .parse::<u32>()
    {
        Ok(number) => {
            println!(
                "line: {}, start: {}, end: {}, number: {}",
                input_line, left_index, right_index, number
            );
            number
        }
        Err(_) => 0,
    }
}
