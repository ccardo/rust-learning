use itertools::{self, Itertools};
use rand::{seq::SliceRandom, thread_rng};
const SUITS: [&str; 4] = ["Hearts", "Diamonds", "Clubs", "Spades"];

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
    for hand in hands {
        let first_iter_hand = hand.into_iter();
        let mut values = first_iter_hand.map(|x| x.value).collect_vec();

        values.sort_by(|&a, b| a.cmp(b));

        let result = duplicate_combinations(&mut values);
        println!("{}", result)
    }
}

struct Card {
    suit: String,
    value: i32,
}

fn create_deck() -> Vec<Card> {
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

fn shuffle_deck(deck: &mut Vec<Card>) -> () {
    deck.shuffle(&mut thread_rng())
}

fn duplicate_combinations(values: &Vec<i32>) -> &str {
    let duplicates = values.into_iter().dedup_with_count().collect_vec();

    if four_of_a_kind(&duplicates) {
        "Poker"
    } else if straight(values) {
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
    }
}

fn two_of_a_kind(duplicate_values: &Vec<(usize, &i32)>) -> bool {
    if duplicate_values.into_iter().any(|x| x.0 == 2) {
        true
    } else {
        false
    }
}

fn double_two_of_a_kind(duplicate_values: &Vec<(usize, &i32)>) -> bool {
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

fn three_of_a_kind(duplicate_values: &Vec<(usize, &i32)>) -> bool {
    if duplicate_values.into_iter().any(|x| x.0 == 3) {
        true
    } else {
        false
    }
}

fn full_house(duplicate_values: &Vec<(usize, &i32)>) -> bool {
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

fn four_of_a_kind(duplicate_values: &Vec<(usize, &i32)>) -> bool {
    if duplicate_values.into_iter().any(|x| x.0 == 4) {
        true
    } else {
        false
    }
}

fn straight(values: &Vec<i32>) -> bool {
    let min_card = values.iter().min().unwrap().to_owned();
    let max_card = values.iter().max().unwrap().to_owned();
    let range = min_card..max_card;
    if values == &range.collect_vec() {
        true
    } else {
        false
    }
}

// complete code with: Flush, Royal Flush. Time how long it takes to get a royal flush.