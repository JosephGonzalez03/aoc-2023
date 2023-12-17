fn main() {
    let input = include_str!("./input-1.txt");
    println!("{}", solution(input));
}

#[derive(Clone, Debug, PartialEq)]
enum Condtion {
    Damaged,
    Unknown
}

#[derive(Debug)]
struct ConditionRecord {
    row: Vec<Vec<Condtion>>,
    damaged_spring_groups: Vec<usize>,
}

fn solution(input: &str) -> usize {
    input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| {
            ConditionRecord {
                row: line
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .first()
                    .unwrap()
                    .split(".")
                    .filter_map(|value| match value.contains(&['#', '?']){
                        true => {
                            Some(value.chars().map(|character| match character {
                                '#' => Condtion::Damaged,
                                '?' => Condtion::Unknown,
                                value => panic!("Invalid spring: {}", value)
                            })
                            .collect::<Vec<Condtion>>())
                        },
                        false => None
                    })
                    .collect::<Vec<Vec<Condtion>>>(),
                damaged_spring_groups: line
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap()
                    .split(",")
                    .collect::<Vec<&str>>()
                    .into_iter()
                    .map(|group_size| group_size.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>(),
            }
        })
        .inspect(|record| println!("record: {:?}", record))
        .map(|record| {
            let mut damage_spring_groups_iterator = record.damaged_spring_groups.iter().clone().peekable();
            let mut springs_subsets: Vec<Vec<Condtion>> = vec![];

            if record.row.len() == 1 {
                let spring_group = record.row.first().unwrap();
                let mut springs_iterator = spring_group.iter().peekable();
                let mut remaining_springs: Vec<Condtion> = spring_group.to_vec();
                let mut current_damage_group = damage_spring_groups_iterator.next().unwrap();

                while let Some (current_spring) = springs_iterator.next() {
                    if !remaining_springs.contains(&Condtion::Damaged) {
                        springs_subsets.push(remaining_springs); 
                        break;
                    }
                    let group_slice: Vec<Condtion> = match remaining_springs.get(..=*current_damage_group as usize) {
                        Some(group_slice) => group_slice.to_vec(),
                        None => break
                    };
                    //println!("{}", group_slice);
                    match current_spring {
                        Condtion::Unknown if group_slice.iter().last().unwrap() == &Condtion::Damaged => remaining_springs = remaining_springs.get(1..).unwrap().to_vec(),
                        Condtion::Unknown | Condtion::Damaged => {
                            springs_subsets.push(group_slice.get(..group_slice.len()-1).unwrap().to_vec()); 
                            remaining_springs = remaining_springs.split_at(*current_damage_group+1).1.to_vec();
                            (0..=*current_damage_group)
                                .into_iter()
                                .for_each(|_| {
                                    springs_iterator.next();
                            });
                            match damage_spring_groups_iterator.next() {
                                Some(next_damage_group) => {
                                    current_damage_group = next_damage_group;
                                },
                                None => break
                            }
                        }
                    }
                }
            } else {
                springs_subsets = record.row;
            }
            println!("subsets: {:?}", springs_subsets);
            
            group_combinations(springs_subsets, record.damaged_spring_groups)
        })
        //.inspect(|value| println!("combinations: {}", value))
        .sum()
}

fn combinations<Rhs>(set: usize, subset: usize) -> usize {
    let set_factorial: usize = (1..=set).product();
    let subset_factorial: usize = (1..=subset).product();
    let difference_subset_factorial: usize = (1..=(set - subset)).product();

    set_factorial / (subset_factorial * difference_subset_factorial)
}

fn group_combinations(group: Vec<Vec<Condtion>>, damaged_groups: Vec<usize>) -> usize {
    let mut damaged_springs_iterator = damaged_groups.iter().peekable();
    let mut results: Vec<usize> = vec![];

    for set in group {
        let mut subset: Vec<&usize> = vec![];
        let mut current_damage_group = match damaged_springs_iterator.next() {
            Some(damage_group) => {
                match set.len() < *damage_group {
                    true => break,
                    false => damage_group
                }
            },
            None => break
        };
        loop {
            subset.push(current_damage_group);
            let required_empty_spaces = subset.len() - 1;
            match damaged_springs_iterator.peek() {
                Some(next_damage_group) => {
                    if set.len()
                        < (subset.clone().into_iter().sum::<usize>()
                            + required_empty_spaces
                            + 1
                            + **next_damage_group)
                    {
                        break;
                    }
                },
                None => break
            }
            match damaged_springs_iterator.next() {
                Some(next_damage_group) => current_damage_group = next_damage_group,
                None => break
            }
        }
        let required_empty_spaces = subset.len() - 1;
        let subset_sum = subset.clone().into_iter().sum::<usize>();
        let result = combinations::<usize>(
            set.len() - required_empty_spaces - subset_sum + subset.len(),
            subset.len(),
        );
        results.push(result);
    }
    results
        .iter()
        .inspect(|value| println!("combination: {}", value))
        .product()
}
