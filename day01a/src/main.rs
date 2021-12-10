use std::fs;
use std::str::FromStr;

fn main() {
    // read file
    let string = fs::read_to_string("./input").expect("Unable to read file.");
    let mut iter = string.split_whitespace();
    let mut prev =  match iter.next() {
        Some(string) => u64::from_str(string).expect("Unable to parse int"),
        None => panic!["Not an integer"]
    };

    let mut count = 0;
    for string in iter {
        let this = u64::from_str(string).expect("Unable to parse int");
        if this > prev {
            count += 1;
        }
        prev = this;
    }

    // print total
    println!("{}", count);
}
