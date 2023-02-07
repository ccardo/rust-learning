use std::{io, process::exit};

fn main() {
    let letters: [char; 4] = ['A', 'C', 'G', 'T'];
    let mut long_seq = String::new();
    let mut short_seq = String::new();
    let germano = long_seq.clone();

    io::stdin().read_line(&mut long_seq).expect("IDIOT");
    io::stdin().read_line(&mut short_seq).expect("idiotino.");

    /*for letter in long_seq.chars() {
        if !(letters.contains(&letter)) {
            println!("The long sequence you inserted is not valid.");
            exit(1)
        }
    }*/

        let (location, count) = return_index(&long_seq, &short_seq);
        println!("The subsequence {} appears at {} for the first time, for a total of {} times",
        short_seq, location.0, count)
    
}

fn return_index<'a>(long_seq: &'a str, short_seq: &str) -> ((usize, &'a str), usize) {

    let matcher: Vec<_> = long_seq.match_indices(&short_seq).collect();
    let counter = matcher.len();
    let first_appearance_idx = matcher[0];

    return (first_appearance_idx, counter);
}