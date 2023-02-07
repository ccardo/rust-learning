use std::io::stdin;

fn main() {
  let mut input = String::new();
  println!("Please input an integer: ");
  stdin().read_line(&mut input).expect("Failed to read line.");
  let x = input.trim().parse::<u32>().unwrap();

  for num in 0..x {
    if armstrong_number(num) {
      println!("{}", num)
    }
  }
}

fn armstrong_number(number: u32) -> bool {
  let number_string = number.to_string();

  let mut sum: u32 = 0;
  for char in number_string.chars() {
    let digit = char.to_digit(10).expect("Failed to convert into an int.");
    sum += digit.pow(number_string.len().try_into().unwrap());
    
    if sum > number { return false }
  }
  
  if sum == number {
    true
  } else { false }
}