fn main() {
    let input = include_str!("./input-1.txt");
    print!("{}", part1(input));
}

struct Scratchcard {
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
}

fn part1(input: &str) -> u32 {
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
            (scratcard
                .numbers_you_have
                .into_iter()
                .filter(|number_you_have| scratcard.winning_numbers.contains(number_you_have))
                .collect::<Vec<u32>>()
                .len()
            ).try_into().unwrap()
        })
        .filter(|number_of_matches| number_of_matches > &0)
        .map(|number_of_matches: u32| (2 as u32).pow(number_of_matches - 1))
        .sum()
}
