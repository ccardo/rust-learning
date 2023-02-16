use rand::{seq::SliceRandom, thread_rng};
use itertools::{self, Itertools};
const SUITS: [&str; 4] = ["Hearts", "Diamonds", "Clubs", "Spades"];

fn main() {
  let mut my_deck = create_deck();
  shuffle_deck(&mut my_deck);

  let mut hands: Vec<Vec<&Card>> = vec![];
  let mut hand: Vec<&Card> = vec![];
  for (i, card) in my_deck.iter().enumerate() {
    hand.push(card);

    let index: i32 = i.try_into().unwrap();
    if index % 5 == 0 {
      hands.push(hand.clone());
      hand.clear()
    }
  }

  let first_hand = &hands[0];
  /* let result = check_combinations(first_hand.to_vec()).unwrap();
  println!("{}", result) */
  let first_iter_hand = first_hand.into_iter();
  let values = first_iter_hand.map(|x| x.value);

  for value in values.clone().collect::<Vec<i32>>() {
    println!("{}", value)
  }

  let mut binding = values.collect_vec();
  binding.sort_by(|&a, b| a.cmp(b));

  for value in &binding {
    println!("{}", value)
  }

  let result = coppia(&binding).expect("No coppia found.");
  println!("{}", result)
}

struct Card {
  suit: String,
  value: i32
}

fn create_deck() -> Vec<Card> {
  let mut deck: Vec<Card> = vec![];

  for suit in SUITS {
    for value in 7..=14 {
      let card: Card = Card {
        suit: suit.to_owned(), 
        value};
      deck.push(card)
    }
  };
  deck
}

fn shuffle_deck(deck: &mut Vec<Card>) -> () {
  deck.shuffle(&mut thread_rng())
}

fn check_combinations(hand: Vec<&Card>) -> Option<&str> {

  let iter_hand = hand.into_iter();
  let values = iter_hand.map(|x| x.value);

  // duplicate values
  let duplicates = values.dedup_with_count();
  if duplicates.clone().count() == 2 {

    Some("Full")
  } else if duplicates.clone().count() == 1 {
    Some("Coppia")
  } else { None }
  // INCORRECT CODE (FIX)
}

fn coppia(hand: &Vec<i32>) -> Option<&str> {

  // do somtething...
  Some("working!")
}