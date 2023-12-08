fn main() {
    let input = include_str!("./input-1.txt");
    print!("{}", part1(input));
}

struct Scratchcard {
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
}

fn part1(input: &str) -> u32 {
    let mut number_of_copied_scratchcards: Vec<u32> = vec![1; input.split_terminator("\n").collect::<Vec<&str>>().len()];
    input
        .split_terminator("\n")
        .map(|line| Scratchcard {
            winning_numbers: line
                .split(": ")
                .collect::<Vec<&str>>()
                .last()
                .expect("There should be a : in the string.")
                .split(" | ")
                .collect::<Vec<&str>>()
                .first()
                .expect("There should be a | in the string.")
                .split(" ")
                .into_iter()
                .filter(|number_you_have| number_you_have != &"")
                .map(|winning_number| {
                    winning_number
                        .parse::<u32>()
                        .expect("This should be a number")
                })
                .collect::<Vec<u32>>(),
            numbers_you_have: line
                .split(": ")
                .collect::<Vec<&str>>()
                .last()
                .expect("There should be a : in the string.")
                .split(" | ")
                .collect::<Vec<&str>>()
                .last()
                .expect("There should be a | in the string.")
                .split(" ")
                .into_iter()
                .filter(|number_you_have| number_you_have != &"")
                .map(|number_you_have| {
                    number_you_have
                        .parse::<u32>()
                        .expect("This should be a number")
                })
                .collect::<Vec<u32>>(),
        })
        .map(|scratcard| {
            scratcard
                .numbers_you_have
                .into_iter()
                .filter(|number_you_have| scratcard.winning_numbers.contains(number_you_have))
                .collect::<Vec<u32>>()
                .len()
        })
        .enumerate()
        .for_each(|(index, number_of_matches)| {
            (index..index+(number_of_matches as usize))
                .into_iter()
                .for_each(|position| {
                    let current_scratchcard_copies = number_of_copied_scratchcards.get(index).expect("The index should exist.");
                    let next_scratchcard_copies = number_of_copied_scratchcards.get(position+1).expect("The index should exist.");
                    number_of_copied_scratchcards[position+1] = current_scratchcard_copies + next_scratchcard_copies;
                });
        });
    number_of_copied_scratchcards
        .into_iter()
        .sum()
}
