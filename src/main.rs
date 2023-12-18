// Rust
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;
use {
    once_cell::sync::Lazy,
    regex::Regex,
};
use core::cmp::Ordering;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Card {
    Joker = 1,
    Ace = 14,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    // Jack = 11, 
    Queen = 12,
    King = 13,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum HandType {
    HighCard = 1,
    Pair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug, Eq, PartialEq, Ord)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub bet: u64,
    pub hand_type: Option<HandType>,
}

impl Hand {
    pub fn new(cards: Vec<Card>, bet: u64) -> Hand {
        Hand {
            cards,
            bet,
            hand_type: None
        }
    }

    pub fn determine_hand_type(&mut self) {
        let mut card_counts = [0; 15];
        let mut jokers = 0;
        for card in &self.cards {
            if card == &Card::Joker {
                jokers += 1;
            }
            else {
                card_counts[*card as usize] += 1;
            }
        }
        let mut max_index = 0;
        let mut max_count = 0;
        for count in 0..card_counts.len() {
            if card_counts[count] > max_count {
                max_count = card_counts[count];
                max_index = count;
            }
        }
        card_counts[max_index] += jokers;

        let mut pair_count = 0;
        let mut three_of_a_kind = false;
        let mut four_of_a_kind = false;
        let mut five_of_a_kind = false;
        for count in &card_counts {
            if *count == 2 {
                pair_count += 1;
            }
            else if *count == 3 {
                three_of_a_kind = true;
            }
            else if *count == 4 {
                four_of_a_kind = true;
            }
            else if *count == 5 {
                five_of_a_kind = true;
            }
        }
        if five_of_a_kind {
            self.hand_type = Some(HandType::FiveOfAKind);
        }
        else if four_of_a_kind {
            self.hand_type = Some(HandType::FourOfAKind);
        }
        else if three_of_a_kind && pair_count == 1 {
            self.hand_type = Some(HandType::FullHouse);
        }
        else if three_of_a_kind {
            self.hand_type = Some(HandType::ThreeOfAKind);
        }
        else if pair_count == 2 {
            self.hand_type = Some(HandType::TwoPair);
        }
        else if pair_count == 1 {
            self.hand_type = Some(HandType::Pair);
        }
        else {
            self.hand_type = Some(HandType::HighCard);
        }
    }
}

fn parse_line(line: &str) -> Hand {
    let mut hand = Hand::new(vec![], 0);
    static SPLIT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\s+").unwrap());
    static CARD: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

    for (idx, cards_part) in SPLIT.split(line).enumerate() {
        if idx == 0 {
            for character in cards_part.chars() {
                match character {
                    'A' => hand.cards.push(Card::Ace),
                    '2' => hand.cards.push(Card::Two),
                    '3' => hand.cards.push(Card::Three),
                    '4' => hand.cards.push(Card::Four),
                    '5' => hand.cards.push(Card::Five),
                    '6' => hand.cards.push(Card::Six),
                    '7' => hand.cards.push(Card::Seven),
                    '8' => hand.cards.push(Card::Eight),
                    '9' => hand.cards.push(Card::Nine),
                    'T' => hand.cards.push(Card::Ten),
                    'J' => hand.cards.push(Card::Joker),
                    'Q' => hand.cards.push(Card::Queen),
                    'K' => hand.cards.push(Card::King),
                    _ => (),
                }
            }
        }
        else {
            hand.bet = u64::from_str(cards_part).unwrap();
        }
    }
    hand.determine_hand_type();
    hand
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.hand_type.as_ref().unwrap() > other.hand_type.as_ref().unwrap() {
            Some(Ordering::Greater)
        }
        else if self.hand_type.as_ref().unwrap() < other.hand_type.as_ref().unwrap() {
            Some(Ordering::Less)
        }
        else {
            for (card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                if card > other_card {
                    return Some(Ordering::Greater);
                }
                else if card < other_card {
                    return Some(Ordering::Less);
                }
            }
            Some(Ordering::Equal)
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input1.txt")?;
    let reader = BufReader::new(file);
    let mut hands = Vec::new();
    let mut score = 0;
    for line in reader.lines() {
        let line = line?;
        let hand = parse_line(&line);
        hands.push(hand);
    }

    hands.sort();
    for (idx, hand) in hands.iter().enumerate() {
        score += hand.bet * (idx + 1) as u64;
    }
    // println!("{:?}", hands);
    println!("{}", score);

    Ok(())
}