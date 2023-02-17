use itertools::{self, Itertools};
use rand::{seq::SliceRandom, thread_rng};
const SUITS: [&str; 4] = ["Hearts", "Diamonds", "Clubs", "Spades"];

#[derive(Debug)]
pub struct Card {
    pub suit: String,
    pub value: i32,
}
fn main() {
    let mut my_deck = create_deck();
    shuffle_deck(&mut my_deck);

    let mut hands: Vec<Vec<&Card>> = vec![];
    let mut hand: Vec<&Card> = vec![];
    for (i, card) in my_deck.iter().enumerate() {
        hand.push(card);

        let index: i32 = i.try_into().unwrap();
        if (index + 1) % 5 == 0 {
            hands.push(hand.clone());
            hand.clear()
        }
    }
    for mut hand in hands {
        let result = combinations(&mut hand);
        println!("{}", result)
    }
}

pub fn create_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];

    for suit in SUITS {
        for value in 8..=14 {
            let card: Card = Card {
                suit: suit.to_owned(),
                value,
            };
            deck.push(card)
        }
    }
    deck
}

pub fn shuffle_deck(deck: &mut Vec<Card>) -> () {
    deck.shuffle(&mut thread_rng())
}

fn combinations(cards: &mut Vec<&Card>) -> String {
    let mut values = cards.into_iter().map(|x| x.value).collect_vec();
    values.sort_by(|&a, b| a.cmp(b)); 
    let mut cloned_values = values.clone();
    let duplicates = (values).into_iter().dedup_with_count().collect_vec();

    let result = if royal_flush(cards){
        "Scala Reale"
    } else if four_of_a_kind(&duplicates) {
        "Poker"
    } else if flush(&cards){
        "Colore"
    } else if straight(&mut cloned_values) {
        "Scala"
    } else if full_house(&duplicates) {
        "Full"
    } else if three_of_a_kind(&duplicates) {
        "Tris"
    } else if double_two_of_a_kind(&duplicates) {
        "Doppia coppia"
    } else if two_of_a_kind(&duplicates) {
        "Coppia"
    } else {
        "None"
    };
    result.to_owned()
}

pub fn two_of_a_kind(duplicate_values: &Vec<(usize, i32)>) -> bool {
    if duplicate_values.into_iter().any(|x| x.0 == 2) {
        true
    } else {
        false
    }
}

pub fn double_two_of_a_kind(duplicate_values: &Vec<(usize, i32)>) -> bool {
    let length = duplicate_values.len();
    if duplicate_values.into_iter().any(|x| x.0 == 2) {
        if length == 3 {
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub fn three_of_a_kind(duplicate_values: &Vec<(usize, i32)>) -> bool {
    if duplicate_values.into_iter().any(|x| x.0 == 3) {
        true
    } else {
        false
    }
}

pub fn full_house(duplicate_values: &Vec<(usize, i32)>) -> bool {
    if two_of_a_kind(duplicate_values) {
        if three_of_a_kind(duplicate_values) {
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub fn four_of_a_kind(duplicate_values: &Vec<(usize, i32)>) -> bool {
    if duplicate_values.into_iter().any(|x| x.0 == 4) {
        true
    } else {
        false
    }
}

pub fn straight(values: &mut Vec<i32>) -> bool {
    let min_card = values.iter().min().unwrap().to_owned();
    let max_card = values.iter().max().unwrap().to_owned();
    let range = min_card..=max_card;
    values.sort_by(|a, b| a.cmp(&b));
    if values == &range.collect_vec() {
        true
    } else {
        false
    }
}

pub fn flush(cards: &Vec<&Card>) -> bool {
    let card_suits = cards.into_iter().map(|&card| &card.suit);
    if card_suits.dedup().count() == 0 {
        true
    } else {
        false
    }
}

pub fn royal_flush(cards: &mut Vec<&Card>) -> bool {
    if flush(cards) {
        if straight(&mut cards.into_iter().map(|card| card.value).collect_vec()) {
            true
        } else {
            false
        }
    } else {
        false
    }
}