fn main() {
    let input = include_str!("./input-1.txt");
    print!("{}", part2(input));
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

fn part2(input: &str) -> u32 {
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
        .map(|game| {
            let mut minimum_number_of_red: u32 = 0;
            let mut minimum_number_of_blue: u32 = 0;
            let mut minimum_number_of_green: u32 = 0;
                    
            game.sets
                .iter()
                .flatten()
                .for_each(|cube| match cube {
                    Cube::Red(amount) => if amount > &minimum_number_of_red {
                        minimum_number_of_red = *amount;
                    },
                    Cube::Blue(amount) => if amount > &minimum_number_of_blue {
                        minimum_number_of_blue = *amount;
                    },
                    Cube::Green(amount) => if amount > &minimum_number_of_green {
                        minimum_number_of_green = *amount;
                    },
                });

            let power_of_cubes = minimum_number_of_red * minimum_number_of_blue * minimum_number_of_green;
            println!("minimum red: {}, minimum blue: {}, minimum green: {}, power: {}", minimum_number_of_red, minimum_number_of_blue, minimum_number_of_green, power_of_cubes);
            power_of_cubes
        })
        .sum()
}
