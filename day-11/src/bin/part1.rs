fn main() {
    let input = include_str!("./input-1.txt");
    println!("{}", solution(input));
}

#[derive(Clone, Debug, PartialEq)]
struct Galaxy {
    row: i32,
    column: i32,
}

fn solution(input: &str) -> u32 {
    let number_of_columns = input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .first()
        .expect("Does exist.")
        .len();
    let empty_row: &str = &".".repeat(number_of_columns);
    let space_image = input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| match !line.contains("#") {
            true => vec![line, empty_row],
            false => vec![line],
        })
        .flatten()
        .collect::<Vec<&str>>();
    let galaxies: Vec<Galaxy> = space_image
        .iter()
        .as_slice()
        .windows(space_image.len())
        .map(|window| {
            let image: Vec<&str> = window.to_vec();
            let empty_columns: Vec<usize> = (0..number_of_columns)
                .into_iter()
                .map(|column_index| {
                    let has_galaxies: Vec<bool> = (0..space_image.len())
                        .into_iter()
                        .map(|row_index| {
                            match window
                                .get(row_index)
                                .expect("Does exist.")
                                .get(column_index..column_index + 1)
                                .expect("Does exist.")
                            {
                                "#" => true,
                                "." => false,
                                value => panic!("Invalid value: {}", value),
                            }
                        })
                        .collect::<Vec<bool>>();
                    match has_galaxies.contains(&true) {
                        true => None,
                        false => Some(column_index),
                    }
                })
                .flatten()
                .collect::<Vec<usize>>();
            image
                .into_iter()
                .map(|row| {
                    let mut modified_row: String = row.to_string();
                    empty_columns
                        .iter()
                        .enumerate()
                        .for_each(|(index, empty_column)| {
                            modified_row.insert(index + *empty_column, '.');
                        });
                    modified_row
                })
                .collect::<Vec<String>>()
        })
        .flatten()
        .enumerate()
        .map(|(row_index, row)| {
            row.chars()
                .enumerate()
                .map(|(column_index, character)| match character {
                    '.' => None,
                    '#' => Some(Galaxy {
                        row: <usize as TryInto<i32>>::try_into(row_index).unwrap(),
                        column: <usize as TryInto<i32>>::try_into(column_index).unwrap(),
                    }),
                    value => panic!("Invalid value: {}", value),
                })
                .flatten()
                .collect::<Vec<Galaxy>>()
        })
        .flatten()
        .collect::<Vec<Galaxy>>();
    let mut galaxy_pairs: Vec<Vec<(&Galaxy, &Galaxy)>> = vec![];
    for mask_index in 1..galaxies.len() {
        let pair = galaxies
            .get(mask_index..galaxies.len())
            .expect("Does exist.")
            .iter()
            .zip(
                galaxies
                    .get(0..galaxies.len() - mask_index)
                    .expect("Does exist.")
                    .iter(),
            )
            .map(|(galaxy1, galaxy2)| (galaxy1, galaxy2))
            .collect::<Vec<(&Galaxy, &Galaxy)>>();
        galaxy_pairs.insert(0, pair);
    }
    galaxy_pairs
        .into_iter()
        .flatten()
        .map(|(galaxy1, galaxy2)| {
            galaxy1.row.abs_diff(galaxy2.row) + galaxy1.column.abs_diff(galaxy2.column)
        })
        .sum()
}
