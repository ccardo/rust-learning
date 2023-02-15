use std::io;
use std::io::{Write};

fn main() {
  let user_input = input("Enter a number:")
      .unwrap();
  let number: u32 = user_input.trim().parse().unwrap();

  if riccardo_number(number).unwrap() {
    println!("{} is a Riccardo number!", number);
  } else { println!("{} is NOT a Riccardo number!", number) };
}

fn riccardo_number(num: u32) -> Option<bool> {
  let num_string = num.to_string();

  let mut total = 0;
  for (i, cipher) in num_string.chars().enumerate() {
    let digit = cipher.to_digit(10).unwrap();
    let position: u32 = i.try_into().unwrap();
    total += digit.pow( position + 1);
  }

  if total != num { return Some(false); }
  Some(true)
}

fn input(message: &str) -> Result<String, io::Error> {
  print!("{}", message);
  io::stdout().flush()?;

  let mut user_input = String::new();
  io::stdin().read_line(&mut user_input)?;

  Ok(user_input)
}