fn main() {
    println!("{};", addup_evens(2, 9));

    let input = String::from("92345789632");
    let x: i128 = input.trim().parse().expect("HEYO");
    let prime = if is_prime(x) { "" } else { "not " };
    println!("The number {} is {}prime.", x, prime)
}

fn addup_evens(bottom: i32, top: i32) -> i32 {
    (bottom..top).filter(|x| x % 2 == 0).sum()
}

fn is_prime(num: i128) -> bool {
    let top = (num as f64).sqrt().floor() as i128;
    for divisor in 2..top {
        if num % divisor == 0 {
            return false;
        }
    }
    return true;
}
