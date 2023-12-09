fn main() {
    let input = include_str!("./input-1.txt");
    println!("{}", part2(input));
}

struct Race {
    time: u64,
    best_distance: u64,
}

fn part2(input: &str) -> u64 {
    *input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .as_slice()
        .chunks_exact(2)
        .map(|chunk| {
            let time = chunk
                .first()
                .expect("Race durations row does exist.")
                .split(":")
                .last()
                .expect("Race duration values do exist.")
                .replace(" ", "")
                .parse::<u64>()
                .expect("Race duration is a number.");
            let best_distance = chunk
                .last()
                .expect("Best distances row does exist.")
                .split(":")
                .last()
                .expect("Best distance values do exist.")
                .replace(" ", "")
                .parse::<u64>()
                .expect("Race duration is a number.");
            vec![Race {
                time,
                best_distance,
            }]
            .into_iter()
            .map(|race| {
                <usize as TryInto<u64>>::try_into(
                    (0..race.time)
                        .into_iter()
                        .map(|time_button_held_down| {
                            time_button_held_down * (race.time - time_button_held_down)
                        })
                        .filter(|race_distance| race_distance > &race.best_distance)
                        .collect::<Vec<u64>>()
                        .len(),
                )
                .unwrap()
            })
            .product()
        })
        .collect::<Vec<u64>>()
        .first()
        .expect("There is always one chunk.")
}
