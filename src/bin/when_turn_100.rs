use std::io::{self, Write};
use chrono::{self, Utc, Datelike};

fn main() {
  let name = input("What is you name? ").expect("Failed to read line");
  println!("Hello, {}!", name);

  let age = input("How old are you? ")
    .expect("Germanio!")
    .trim().parse::<i32>()
    .expect("Failed to convert into integer.");

  let current_year = Utc::now().year();
  let year_of_100 = current_year + 100 - age;
  println!("You will turn 100 in {}", year_of_100)
}

fn input(message: &str) -> Result<String, io::Error> {
  print!("{}", message);
  io::stdout().flush()?;

  let mut user_input = String::new();
  io::stdin().read_line(&mut user_input)?;

  Ok(user_input.trim().to_owned())
}