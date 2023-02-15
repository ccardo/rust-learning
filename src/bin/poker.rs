use rand::{seq::SliceRandom, thread_rng};
const SUITS: [&str; 4] = ["Hearts", "Diamonds", "Clubs", "Spades"];

fn main() {
  let mut my_deck = create_deck();
  shuffle_deck(&mut my_deck)
}

struct Card {
  suit: String,
  value: i32
}

fn create_deck() -> Vec<Card> {
  let mut deck: Vec<Card> = vec![];

  for suit in SUITS {
    for value in 1..14 {
      let card: Card = Card {
        suit: suit.to_owned(),
        value
      };
      deck.push(card)
    }
  };
  deck
}

fn shuffle_deck(deck: &mut Vec<Card>) -> () {
  deck.shuffle(&mut thread_rng())
}