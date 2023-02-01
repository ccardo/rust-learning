use std::io;

fn main() {

    let mut _contents = String::new();
    let mut people_left: u32 = 7;
    println!("There are {people_left} people left");

    while 0 < people_left {

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read input.");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        people_left += input as u32;
        println!("\nThere are {people_left} people left.");
    };
}