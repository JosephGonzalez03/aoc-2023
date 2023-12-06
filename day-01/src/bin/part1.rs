fn main() {
    let input = include_str!("./input-1.txt");
    print!("{}", part1(input));
}

fn part1(input: &str) -> u32 {
    input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| {
            line.chars()
                .filter(|character| character.is_digit(10))
                .collect::<Vec<char>>()
        })
        .map(|line_digits: Vec<char>| {
            match (line_digits.get(0).expect("Exists").to_string()
                + &line_digits.last().expect("Exists").to_string())
                .parse::<u32>()
            {
                Ok(number) => Some(number),
                _ => None,
            }
        })
        .flatten()
        .sum()
}
