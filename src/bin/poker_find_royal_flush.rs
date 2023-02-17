mod poker;
use poker::{Card, create_deck, royal_flush, shuffle_deck};

fn main() {

  let mut n: u32 = 1;
  'search:loop {

    n += 1;
    let mut deck = create_deck();
    shuffle_deck(&mut deck);
  
    let mut hands: Vec<Vec<&Card>> = vec![];
    let mut hand: Vec<&Card> = vec![];
  
    for (i, card) in deck.iter().enumerate() {
      hand.push(card);
  
      if (i + 1) % 5 == 0 {
        hands.push(hand.clone());
        hand.clear();
      }
    }

    for mut hand in hands {
      if royal_flush(&mut hand) {
        println!("Scala Reale! It took exactly {n} decks.");
        break 'search
      }
    }
  }
}