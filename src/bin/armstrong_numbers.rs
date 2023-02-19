use std::io;

fn main() {
  let mut input = String::new();
  println!("Please input an integer: ");
  io::stdin().read_line(&mut input).expect("Failed to read line.");
  let x = input.trim().parse::<u32>().unwrap();
  let mut n = 1;
  
  for num in 0..x {
    if armstrong_number(num).unwrap() {
      println!("{}", num);
      n += 1;
    }
  }
  println!("There are a total of {n} Armstrong numbers up to {x}.")
}

fn armstrong_number(number: u32) -> Option<bool> {
  let number_string = number.to_string();

  let mut sum: u32 = 0;
  for char in number_string.chars() {
    let digit = char.to_digit(10)?;
    sum += digit.pow(number_string.len().try_into().unwrap());
  
    if sum > number { return Some(false) }
  }
  
  if sum == number {
    Some(true)
  } else { Some(false) }
}