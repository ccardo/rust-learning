use std::io;

fn main() {

    let mut _contents = String::new();
    let mut chapters_left: i32 = 9;
    println!("There are {chapters_left} chapters left");

    while 0 < chapters_left {

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read input.");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };

        chapters_left -= input;
        println!("\nThere are {chapters_left} chapters left.");
    };
}