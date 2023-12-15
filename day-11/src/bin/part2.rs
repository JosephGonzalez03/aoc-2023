fn main() {
    let input = include_str!("./input-1.txt");
    println!("{}", solution(input));
}

#[derive(Clone, Debug, PartialEq)]
struct Galaxy {
    row: i64,
    column: i64,
}

fn solution(input: &str) -> u64 {
    let number_of_rows = input.split_terminator("\n").collect::<Vec<&str>>().len();
    let number_of_columns = input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .first()
        .expect("Does exist.")
        .len();
    let empty_rows = input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .enumerate()
        .map(|(index, line)| match !line.contains("#") {
            true => Some(index),
            false => None,
        })
        .flatten()
        .collect::<Vec<usize>>();
    let empty_columns: Vec<usize> = input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .iter()
        .as_slice()
        .windows(number_of_rows)
        .map(|window| {
            let empty_columns: Vec<usize> = (0..number_of_columns)
                .into_iter()
                .map(|column_index| {
                    let has_galaxies: Vec<bool> = (0..number_of_rows)
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
            empty_columns
        })
        .flatten()
        .collect::<Vec<usize>>();

    let galaxies: Vec<Galaxy> = input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .enumerate()
        .map(|(row_index, row)| {
            row.chars()
                .enumerate()
                .map(|(column_index, character)| match character {
                    '.' => None,
                    '#' => Some(Galaxy {
                        row: <usize as TryInto<i64>>::try_into(row_index).unwrap(),
                        column: <usize as TryInto<i64>>::try_into(column_index).unwrap(),
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
            let empty_rows_between_x_delta = <usize as TryInto<u64>>::try_into(
                empty_rows
                    .clone()
                    .into_iter()
                    .filter(|empty_row_index| match galaxy1.row - galaxy2.row {
                        difference if difference > 0 => {
                            empty_row_index < &(galaxy1.row as usize)
                                && empty_row_index > &(galaxy2.row as usize)
                        }
                        _ => {
                            empty_row_index < &(galaxy2.row as usize)
                                && empty_row_index > &(galaxy1.row as usize)
                        }
                    })
                    .collect::<Vec<usize>>()
                    .len(),
            )
            .unwrap();
            let empty_columns_between_y_delta = <usize as TryInto<u64>>::try_into(
                empty_columns
                    .clone()
                    .into_iter()
                    .filter(|empty_column_index| match galaxy1.column - galaxy2.column {
                        difference if difference > 0 => {
                            empty_column_index < &(galaxy1.column as usize)
                                && empty_column_index > &(galaxy2.column as usize)
                        }
                        _ => {
                            empty_column_index < &(galaxy2.column as usize)
                                && empty_column_index > &(galaxy1.column as usize)
                        }
                    })
                    .collect::<Vec<usize>>()
                    .len(),
            )
            .unwrap();
            // 1000000
            ((999999 * empty_rows_between_x_delta) + galaxy1.row.abs_diff(galaxy2.row))
                + ((999999 * empty_columns_between_y_delta)
                    + galaxy1.column.abs_diff(galaxy2.column))
        })
        .sum()
}
