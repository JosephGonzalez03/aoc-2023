fn main() {
    let input = include_str!("./input-1.txt");
    println!("{}", part1(input));
}

struct Race {
    time: u32,
    best_distance: u32,
}

fn part1(input: &str) -> u32 {
    *input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .as_slice()
        .chunks_exact(2)
        .map(|chunk| {
            let times = chunk
                .first()
                .expect("Race durations row does exist.")
                .split(":")
                .last()
                .expect("Race duration values do exist.")
                .split_whitespace()
                .into_iter()
                .map(|time| time.parse::<u32>().expect("Race duration is a number."))
                .collect::<Vec<u32>>();
            let best_distances = chunk
                .last()
                .expect("Best distances row does exist.")
                .split(":")
                .last()
                .expect("Best distance values do exist.")
                .split_whitespace()
                .into_iter()
                .map(|time| time.parse::<u32>().expect("Race duration is a number."))
                .collect::<Vec<u32>>();

            times
                .into_iter()
                .zip(best_distances.into_iter())
                .map(|(time, best_distance)| Race {
                    time,
                    best_distance,
                })
                .map(|race| {
                    <usize as TryInto<u32>>::try_into(
                        (0..race.time)
                            .into_iter()
                            .map(|time_button_held_down| {
                                time_button_held_down * (race.time - time_button_held_down)
                            })
                            .filter(|race_distance| race_distance > &race.best_distance)
                            .collect::<Vec<u32>>()
                            .len(),
                    )
                    .unwrap()
                })
                .product()
        })
        .collect::<Vec<u32>>()
        .first()
        .expect("There is always one chunk.")
}
