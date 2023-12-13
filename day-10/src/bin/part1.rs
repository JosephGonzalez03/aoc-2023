use std::ops::Div;

fn main() {
    let input = include_str!("./input-1.txt");
    println!("{}", solution(input));
}

#[derive(Debug, PartialEq)]
enum Pipe {
    NorthAndSouth,
    EastAndWest,
    NorthAndEast,
    NorthAndWest,
    SouthAndWest,
    SouthAndEast,
    Start,
}

#[derive(Debug, PartialEq)]
struct Tile {
    pipe: Pipe,
    row: i32,
    column: i32,
}

fn solution(input: &str) -> i32 {
    let tiles: Vec<Tile> = input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .enumerate()
        .map(|(line_index, line)| {
            line.match_indices(|character| match character {
                '|' | '-' | 'L' | 'J' | '7' | 'F' | 'S' => true,
                _ => false,
            })
            .map(|(match_index, matched_symbol)| Tile {
                pipe: match matched_symbol {
                    "|" => Pipe::NorthAndSouth,
                    "-" => Pipe::EastAndWest,
                    "L" => Pipe::NorthAndEast,
                    "J" => Pipe::NorthAndWest,
                    "7" => Pipe::SouthAndWest,
                    "F" => Pipe::SouthAndEast,
                    "S" => Pipe::Start,
                    value => panic!("Invalida character: {}", value),
                },
                row: <usize as TryInto<i32>>::try_into(line_index).unwrap(),
                column: <usize as TryInto<i32>>::try_into(match_index).unwrap(),
            })
            .collect::<Vec<Tile>>()
        })
        .flatten()
        .collect::<Vec<Tile>>();
    let mut current_tile: &Tile = tiles
        .iter()
        .filter(|tile| tile.pipe == Pipe::Start)
        .collect::<Vec<&Tile>>()
        .first()
        .expect("The tile does exist.");
    let mut tarversed_tiles: Vec<&Tile> = vec![current_tile];
    let north_connecting_pipes = vec![Pipe::SouthAndWest, Pipe::NorthAndSouth, Pipe::SouthAndEast];
    let east_connecting_pipes = vec![Pipe::NorthAndWest, Pipe::EastAndWest, Pipe::SouthAndWest];
    let south_connecting_pipes = vec![Pipe::NorthAndWest, Pipe::NorthAndSouth, Pipe::NorthAndEast];
    let west_connecting_pipes = vec![Pipe::NorthAndEast, Pipe::EastAndWest, Pipe::SouthAndEast];

    loop {
        tarversed_tiles.push(current_tile);
        match (match current_tile.pipe {
            Pipe::NorthAndSouth => tiles
                .iter()
                .filter(|tile| {
                    (
                        (tile.row == current_tile.row - 1
                            && tile.column == current_tile.column
                            && north_connecting_pipes.contains(&tile.pipe))
                        || ((tile.row == current_tile.row + 1 && tile.column == current_tile.column)
                            && south_connecting_pipes.contains(&tile.pipe))
                    )
                    && !tarversed_tiles.contains(tile)
                })
                .collect::<Vec<&Tile>>(),
            Pipe::EastAndWest => tiles
                .iter()
                .filter(|tile| {
                    (
                        (tile.row == current_tile.row
                            && tile.column == current_tile.column + 1
                            && east_connecting_pipes.contains(&tile.pipe))
                        || (tile.row == current_tile.row
                            && tile.column == current_tile.column - 1)
                            && west_connecting_pipes.contains(&tile.pipe)
                    )
                    && !tarversed_tiles.contains(tile)
                })
                .collect::<Vec<&Tile>>(),
            Pipe::NorthAndEast => tiles
                .iter()
                .filter(|tile| {
                    (
                        (tile.row == current_tile.row - 1
                            && tile.column == current_tile.column
                            && north_connecting_pipes.contains(&tile.pipe))
                        || (tile.row == current_tile.row
                            && tile.column == current_tile.column + 1
                            && east_connecting_pipes.contains(&tile.pipe))
                    )
                    && !tarversed_tiles.contains(tile)
                })
                .collect::<Vec<&Tile>>(),
            Pipe::NorthAndWest => tiles
                .iter()
                .filter(|tile| {
                    (
                        (tile.row == current_tile.row - 1
                            && tile.column == current_tile.column
                            && north_connecting_pipes.contains(&tile.pipe))
                        || (tile.row == current_tile.row
                            && tile.column == current_tile.column - 1
                            && west_connecting_pipes.contains(&tile.pipe))
                    )
                    && !tarversed_tiles.contains(tile)
                })
                .collect::<Vec<&Tile>>(),
            Pipe::SouthAndWest => tiles
                .iter()
                .filter(|tile| {
                    (
                        (tile.row == current_tile.row + 1
                            && tile.column == current_tile.column
                            && south_connecting_pipes.contains(&tile.pipe))
                        || (tile.row == current_tile.row
                            && tile.column == current_tile.column - 1
                            && west_connecting_pipes.contains(&tile.pipe))
                    )
                    && !tarversed_tiles.contains(tile)
                })
                .collect::<Vec<&Tile>>(),
            Pipe::SouthAndEast => tiles
                .iter()
                .filter(|tile| {
                    (
                        (tile.row == current_tile.row + 1
                            && tile.column == current_tile.column
                            && south_connecting_pipes.contains(&tile.pipe))
                        || (tile.row == current_tile.row
                            && tile.column == current_tile.column + 1
                            && east_connecting_pipes.contains(&tile.pipe))
                    )
                    && !tarversed_tiles.contains(tile)
                })
                .collect::<Vec<&Tile>>(),
            Pipe::Start => tiles
                .iter()
                .filter(|tile| {
                    (
                        (tile.row == current_tile.row - 1
                            && tile.column == current_tile.column
                            && north_connecting_pipes.contains(&tile.pipe))
                        || (tile.row == current_tile.row + 1
                               && tile.column == current_tile.column
                               && south_connecting_pipes.contains(&tile.pipe))
                        || (tile.row == current_tile.row
                               && tile.column == current_tile.column - 1
                               && west_connecting_pipes.contains(&tile.pipe))
                        || (tile.row == current_tile.row
                               && tile.column == current_tile.column + 1
                               && east_connecting_pipes.contains(&tile.pipe))
                    )
                    && !tarversed_tiles.contains(tile)
                })
                .collect::<Vec<&Tile>>(),
        })
        .first()
        {
            Some(next_tile) => current_tile = next_tile,
            None => break,
        }
    }
    let path_length: i32 = <usize as TryInto<i32>>::try_into(tarversed_tiles.len()).unwrap();

    match path_length % 2 {
        0 => path_length.div(2),
        _ => path_length.div(2),
    }
}
