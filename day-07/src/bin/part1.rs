use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input-1.txt");
    println!("{}", part1(input));
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Eq, Ord, Hash, PartialEq)]
struct Hand(Vec<Card>);

struct Player {
    hand: Hand,
    bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match hand_type(&self.0).cmp(&hand_type(&other.0)) {
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => self.0
                    .iter()
                    .zip(other.0.iter())
                    .map(|(self_card, other_card)| self_card.partial_cmp(&other_card))
                    .flatten()
                    .filter(|comparison| comparison != &Ordering::Equal)
                    .collect::<Vec<Ordering>>()
                    .first()
                    .copied(),
            }
        }
}

fn hand_type(hand: &Vec<Card>) -> HandType {
    let mut card_map = HashMap::new();
    hand.iter().for_each(|card| {
        if !card_map.contains_key(&card) {
            card_map.insert(card, 0);
        }

        if let Some(occurances) = card_map.get_mut(&card) {
            *occurances = *occurances + 1;
        }
    });
    match card_map
        .values()
        .into_iter()
        .max()
        .expect("The hand is not empty.")
    {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => match card_map
            .values()
            .into_iter()
            .min()
            .expect("There will be a minimum value.")
        {
            2 => HandType::FullHouse,
            1 => HandType::ThreeOfAKind,
            _ => panic!("The hand has more than 5 cards."),
        },
        2 => match card_map
            .values()
            .into_iter()
            .collect::<Vec<&u32>>()
            .len()
        {
            3 => HandType::TwoPair,
            4 => HandType::OnePair,
            _ => panic!("The hand has more than 5 cards."),
        },
        1 => HandType::HighCard,
        _ => panic!("The hand has more than 5 cards."),
    }
}

fn part1(input: &str) -> u32 {
    let mut players = input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| Player {
            hand: Hand(line
                .split(" ")
                .collect::<Vec<&str>>()
                .first()
                .expect("Hand does exist.")
                .chars()
                .map(|card| match card {
                    '2' => Card::Two,
                    '3' => Card::Three,
                    '4' => Card::Four,
                    '5' => Card::Five,
                    '6' => Card::Six,
                    '7' => Card::Seven,
                    '8' => Card::Eight,
                    '9' => Card::Nine,
                    'T' => Card::Ten,
                    'J' => Card::Jack,
                    'Q' => Card::Queen,
                    'K' => Card::King,
                    'A' => Card::Ace,
                    _ => panic!("Invalid card."),
                })
                .collect::<Vec<Card>>()),
            bid: line
                .split(" ")
                .collect::<Vec<&str>>()
                .last()
                .expect("Bid does exist.")
                .trim()
                .parse::<u32>()
                .expect("Bid is a number."),
        })
        .collect::<Vec<Player>>();
    players.sort_by(|player1, player2| player1.hand.partial_cmp(&player2.hand).expect("The hands are comparable."));
    players
        .into_iter()
        .enumerate()
        .map(|(index, player)| player.bid * (index as u32 + 1))
        .sum()
}
