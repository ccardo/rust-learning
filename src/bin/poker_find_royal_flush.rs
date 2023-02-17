mod poker;
use itertools::Itertools;
use poker::{Card, create_deck, royal_flush, shuffle_deck};

fn main() {

  /* 'search:loop { */
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

    let card1 = Card { suit: "Diamonds".to_string(), value: 14 };
    let card2 = Card { suit: "Diamonds".to_string(), value: 13 };
    let card3 = Card { suit: "Diamonds".to_string(), value: 11 };
    let card4 = Card { suit: "Diamonds".to_string(), value: 12 };
    let card5 = Card { suit: "Diamonds".to_string(), value: 10 };
    let mut germano = vec![
    &card1,
    &card2,
    &card3,
    &card4,
    &card5,
    ];
    
    let mut ohoho = germano.iter().map(|x| x.value).collect_vec();
    let result = royal_flush(&mut germano);

    println!("{}", result);

    /*   let results = hands
      .iter()
      .map(|hand| royal_flush(hand))
      .collect::<Vec<bool>>();

      if results.iter().any(|&result| result) {
        println!("Scala Reale");
        break 'search
    } */
  /* } */
}