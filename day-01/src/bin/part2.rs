fn main() {
    let input = include_str!("./input-2.txt");
    print!("{}", part2(input));
}

fn part2(input: &str) -> u32 {
    input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| {
            let mut altered_line = String::from(line);
            let mut numbers: Vec<(usize, &str)> = line
                .match_indices("one")
                .chain(line.match_indices("two"))
                .chain(line.match_indices("three"))
                .chain(line.match_indices("four"))
                .chain(line.match_indices("five"))
                .chain(line.match_indices("six"))
                .chain(line.match_indices("seven"))
                .chain(line.match_indices("eight"))
                .chain(line.match_indices("nine"))
                .collect();
            numbers.sort_by(|(index_a, _), (index_b, _)| index_a.cmp(index_b));
            numbers.iter().for_each(|(_, number)| {
                if let Some((new_index, _)) = altered_line
                    .clone()
                    .match_indices(number)
                    .collect::<Vec<(usize, &str)>>()
                    .first()
                {
                    altered_line.replace_range(
                        new_index..&(new_index + number.len() - 1),
                        match *number {
                            "one" => "1",
                            "two" => "2",
                            "three" => "3",
                            "four" => "4",
                            "five" => "5",
                            "six" => "6",
                            "seven" => "7",
                            "eight" => "8",
                            _ => "9",
                        },
                    );
                }
            });
            let result = altered_line
                .chars()
                .filter(|character| character.is_digit(10))
                .collect::<Vec<char>>();
            println!("numbers: {:?}", numbers);
            println!("result: {:?}", result);
            result
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
