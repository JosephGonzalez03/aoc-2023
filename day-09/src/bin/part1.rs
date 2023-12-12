fn main() {
    let input = include_str!("./input-1.txt");
    println!("{}", solution(input));
}

fn solution(input: &str) -> i32 {
    input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|history| {
            history
                .split(" ")
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|history_value| {
                    history_value
                        .parse::<i32>()
                        .expect("The history value is a number.")
                })
                .collect::<Vec<i32>>()
        })
        .map(|history| {
            let mut difference_sequences: Vec<Vec<i32>> = vec![history.clone()];
            let mut next_difference_sequence: Vec<i32> = history;

            loop {
                let current_difference_sequence: Vec<i32> = next_difference_sequence
                    .as_slice()
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect::<Vec<i32>>();
                difference_sequences.push(current_difference_sequence.clone());
                next_difference_sequence = current_difference_sequence.clone();
                if current_difference_sequence
                    .iter()
                    .filter(|values| values == &&0)
                    .collect::<Vec<&i32>>()
                    .len()
                    == current_difference_sequence.len()
                {
                    break;
                }
            }
            difference_sequences.reverse();
            difference_sequences
                .first_mut()
                .expect("There will be more than 0 difference sequences.")
                .push(0);
            println!("{:?}", difference_sequences);
            difference_sequences
                .as_slice()
                .windows(difference_sequences.len())
                .map(|window| {
                    window
                        .into_iter()
                        .map(|difference_sequence| {
                            difference_sequence
                                .last()
                                .expect("This does exist.")
                                .to_owned()
                        })
                        .collect::<Vec<i32>>()
                })
                .flatten()
                .collect::<Vec<i32>>()
        })
        .flatten()
        .sum::<i32>()
}
