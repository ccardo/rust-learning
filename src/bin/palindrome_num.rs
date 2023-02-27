use std::collections::HashMap;

fn main() {
    let s = "LVIII";
    let dict:HashMap<char, i32> = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000)
    ]);

    let mut total = 0;
    let mut previous_value =  0;
    for c in s.chars() {
        let value = *dict.get(&c).unwrap();
        total += value;

        if value > previous_value {
            total -= 2 * previous_value
        };
        previous_value = value;
    }
    println!("{}", total)
}