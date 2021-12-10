use std::fs;
use std::str::FromStr;

fn main() {
    let string = fs::read_to_string("./input").expect("Read failed");
    let data: Vec<u64> = string.split_whitespace()
                               .map(|s| u64::from_str(s).expect("Parse failed"))
                               .collect();
    let mut count = 0;
    for i in 3..data.len() {
        let prev: u64 = data[(i-3)..(i+0)].iter().sum();
        let this: u64 = data[(i-2)..(i+1)].iter().sum();
        if this > prev {
            // println!["{}, {}: increased", prev, this];
            count += 1;
        } else {
            // println!["{}, {}", prev, this];
        }
    }
    println!["{}", count];
}
