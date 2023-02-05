use std::convert::TryInto;
use std::io::stdin;

fn main() {
    println!("{};", addup_evens(2, 9));

    let mut x = String::new();
    stdin().read_line(&mut x).expect("HEY");

    let x: i128 = x.trim().parse().expect("Failed to convert into integer.");

    let prime = if is_prime(x as i128) { "" } else { "not " };
    println!("The number {} is {}prime.", x, prime);

    println!("The number of primes up to {} is: {}", x, prime_sieve(x.try_into().unwrap()))
}

fn addup_evens(bottom: i32, top: i32) -> i32 {
    (bottom..top).filter(|x| x % 2 == 0).sum()
}

fn is_prime(num: i128) -> bool {
    let top = (num as f64).sqrt().ceil() as i128;
    for divisor in 2..(top + 1) {
        if num % divisor == 0 {
            return false;
        }
    }
    return true;
}

fn prime_sieve(max_number_to_check: usize) -> u32 {
    let mut prime_mask = vec![true; max_number_to_check];
    prime_mask[0] = false;
    prime_mask[1] = false;

    let mut total_primes_found = 0;

    const FIRST_PRIME_NUMBER: usize = 2;
    for p in FIRST_PRIME_NUMBER..max_number_to_check {
        if prime_mask[p] {
            
            total_primes_found += 1;
            let mut i = 2 * p;
            while i < max_number_to_check {
                prime_mask[i] = false;
                i += p;
            }
        }
    }

    total_primes_found
}