use rayon::prelude::*;

fn main() {
    let input = include_str!("./input-1.txt");
    println!("{}", part2(input));
}

struct ConversionMapIndices {
    start_index: usize,
    end_index: usize,
}

#[derive(Debug)]
struct ConversionMap {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

fn part2(input: &str) -> u64 {
    let conversion_maps = input
        .split_terminator("\n")
        .filter(|line| line != &"")
        .enumerate()
        .map(|(index, line)| if line.contains("map") { index } else { 0 })
        .filter(|index| index != &0)
        .collect::<Vec<usize>>()
        .as_slice()
        .windows(2)
        .map(|window| ConversionMapIndices {
            start_index: *window.get(0).expect("Should exist.") + 1,
            end_index: *window.get(1).expect("Should exist."),
        })
        .map(|conversion_map_indices| {
            input
                .split_terminator("\n")
                .filter(|line| line != &"")
                .collect::<Vec<&str>>()
                .get(conversion_map_indices.start_index..conversion_map_indices.end_index)
                .expect("The map indices guarantees this exists.")
                .into_iter()
                .map(|line| line
                     .split(" ")
                     .map(|map_value| map_value.trim().parse::<u64>().expect("Map value should be a number."))
                     .collect::<Vec<u64>>()
                )
                .map(|parsed_map_values| {
                    let [destination_range_start, source_range_start, range_length] = parsed_map_values
                        .get(..3)
                        .expect("A row should have 3 values.")
                    else {
                        panic!("The map values are not in their expected format.")
                    };
                    ConversionMap {
                        destination_range_start: *destination_range_start,
                        source_range_start: *source_range_start,
                        range_length: *range_length
                    }
                })
                .collect::<Vec<ConversionMap>>()
        })
        .collect::<Vec<Vec<ConversionMap>>>();
    let mut locations = input
        .split_terminator("\n")
        .filter(|line| line != &"")
        .collect::<Vec<&str>>()
        .first()
        .expect("First line does exist.")
        .split(": ")
        .collect::<Vec<&str>>()
        .last()
        .expect("The seeds are after the colon.")
        .split(" ")
        .map(|seed| seed.trim()
                .parse::<u64>()
                .expect("Seed should be a number.")
        )
        .collect::<Vec<u64>>()
        .as_slice()
        .chunks(2)
        .map(|chunk| {
            let [first, second] = chunk.get(..2).expect("These should exist.") else { panic!("Seeds are not in expected format.") };
            let seeds = (*first..(*first+*second))
                .into_iter()
                .map(|seed| seed)
                .collect::<Vec<u64>>();
            seeds
                .par_iter()
                .map(|seed| {
                     let mut output: u64 = *seed;
                     conversion_maps
                         .iter()
                         .for_each(|conversion_map| {
                             match conversion_map
                                 .into_iter()
                                 .filter(|conversion_range|
                                         output >= conversion_range.source_range_start &&
                                         output < conversion_range.source_range_start + conversion_range.range_length)
                                 .collect::<Vec<&ConversionMap>>()
                                 .first() {
                                     Some(conversion_range) => {
                                         output = conversion_range.destination_range_start + (output - conversion_range.source_range_start);
                                     },
                                     None => (),
                                 }
                         });
                     output
                })
                .min()
                .expect("There will be a value.")
        })
        .collect::<Vec<u64>>();

    locations.sort();
    *locations
        .first()
        .expect("The locations list will not be empty.")
}
