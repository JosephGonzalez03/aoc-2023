use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input-1.txt");
    println!("{}", part2(input));
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
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Eq)]
struct Player {
    hand: Vec<Card>,
    bid: u32,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        match self
            .hand
            .iter()
            .zip(other.hand.iter())
            .map(|(self_card, other_card)| self_card.cmp(&other_card))
            .filter(|ordering| ordering != &Ordering::Equal)
            .collect::<Vec<Ordering>>()
            .first()
        {
            Some(_) => false,
            None => true,
        }
    }
}

impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        match self
            .hand
            .iter()
            .zip(other.hand.iter())
            .map(|(self_card, other_card)| self_card.cmp(&other_card))
            .filter(|ordering| ordering != &Ordering::Equal)
            .collect::<Vec<Ordering>>()
            .first()
        {
            Some(ordering) => *ordering,
            None => Ordering::Equal,
        }
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match hand_type(&self.hand).cmp(&hand_type(&other.hand)) {
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => self.hand
                    .iter()
                    .zip(other.hand.iter())
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
    println!("");
    println!("{:?}", card_map);
    if let Some(joker_occurances) = card_map.remove(&Card::Joker) {
        let (mut highest_card, mut highest_occurances) = (&Card::Joker, 0);
        
        card_map
            .iter()
            .for_each(|(card, occurances)| {
                if occurances > &highest_occurances {
                    highest_card = card;
                    highest_occurances = *occurances;
                } else if occurances >= &highest_occurances && card > &highest_card {
                    highest_card = card;
                    highest_occurances = *occurances;
                }
            });
        card_map.insert(&highest_card, highest_occurances + joker_occurances);
        println!("{:?}", card_map);
    } 
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

fn part2(input: &str) -> u32 {
    let mut players = input
        .split_terminator("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| Player {
            hand: line
                .split(" ")
                .collect::<Vec<&str>>()
                .first()
                .expect("Hand does exist.")
                .chars()
                .map(|card| match card {
                    'J' => Card::Joker,
                    '2' => Card::Two,
                    '3' => Card::Three,
                    '4' => Card::Four,
                    '5' => Card::Five,
                    '6' => Card::Six,
                    '7' => Card::Seven,
                    '8' => Card::Eight,
                    '9' => Card::Nine,
                    'T' => Card::Ten,
                    'Q' => Card::Queen,
                    'K' => Card::King,
                    'A' => Card::Ace,
                    _ => panic!("Invalid card."),
                })
                .collect::<Vec<Card>>(),
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
    players.sort();
    players.iter().for_each(|player| println!("{:?}", player.hand));
    players
        .into_iter()
        .enumerate()
        .map(|(index, player)| player.bid * (index as u32 + 1))
        .sum()
}
