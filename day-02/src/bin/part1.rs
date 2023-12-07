fn main() {
    let input = include_str!("./input-1.txt");
    print!("{}", part1(input));
}

enum Cube {
    Red(u32),
    Blue(u32),
    Green(u32),
}

struct Game {
    id: u32,
    sets: Vec<Vec<Cube>>,
}

fn part1(input: &str) -> u32 {
    input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| Game {
            id: line
                .split(":")
                .collect::<Vec<&str>>()
                .first()
                .expect("First item should exist.")
                .split(" ")
                .collect::<Vec<&str>>()
                .last()
                .expect("This should be the game id.")
                .parse::<u32>()
                .expect("The game id should be a number."),
            sets: line
                .split(":")
                .collect::<Vec<&str>>()
                .last()
                .expect("Last item should exist.")
                .trim()
                .split(";")
                .into_iter()
                .map(|set| {
                    set.split(",")
                        .collect::<Vec<&str>>()
                        .into_iter()
                        .map(|cubes| {
                            let number_of_cubes = cubes
                                .trim()
                                .split(" ")
                                .collect::<Vec<&str>>()
                                .first()
                                .expect("Number should exist.")
                                .parse::<u32>()
                                .expect("This should be a number.");
                            let cubes_color = cubes
                                .trim()
                                .split(" ")
                                .collect::<Vec<&str>>()
                                .last()
                                .expect("Color should exist.")
                                .to_owned();

                            match cubes_color {
                                "red" => Cube::Red(number_of_cubes),
                                "blue" => Cube::Blue(number_of_cubes),
                                _ => Cube::Green(number_of_cubes),
                            }
                        })
                        .collect::<Vec<Cube>>()
                })
                .collect::<Vec<Vec<Cube>>>(),
        })
        .filter(|game| {
            !game
                .sets
                .iter()
                .flatten()
                .map(|cube| match cube {
                    Cube::Red(amount) => amount > &12,
                    Cube::Blue(amount) => amount > &14,
                    Cube::Green(amount) => amount > &13,
                })
                .collect::<Vec<bool>>()
                .contains(&true)
        })
        .map(|possible_game| possible_game.id)
        .sum()
}
