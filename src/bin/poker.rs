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

    let first_hand = &hands[0];
    let first_iter_hand = first_hand.into_iter();
    let values = first_iter_hand.map(|x| x.value);

    let mut binding = values.collect_vec();
    binding.sort_by(|&a, b| a.cmp(b));

    let result = match duplicati(&mut binding) {
        Some(coppia) => coppia,
        None => ""
    };
    println!("{}", result)
}

struct Card {
    suit: String,
    value: i32,
}

fn create_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];

    for suit in SUITS {
        for value in 7..=14 {
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

fn duplicati(values: &mut Vec<i32>) -> Option<&str> {
    let duplicates = values.into_iter().dedup_with_count().collect_vec();

    for i in 2..=4 {
        let boolean = if (&duplicates).into_iter()
            .any(|x| x.0 == i) { true } else {false};

        if boolean {
            return match i {
                4 => Some("Poker"),
                3 => Some("Tris"),
                2 => {return match duplicates.len() {
                    4 => Some("Coppia"),
                    3 => Some("Doppia Coppia"),
                    2 => Some("Full Hand"),
                    _ => None};}
                _ => None
            };
        }
    }
    None
}