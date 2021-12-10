use std::fs;
use std::str::FromStr;
use regex::Regex;

fn main() {
    let text = fs::read_to_string("./input").expect("Read failed");
    let re = Regex::new(r"([a-z]+) (\d+)").expect("Regex failed");
    let mut depth = 0;
    let mut position = 0;
    let mut aim = 0;
    for caps in re.captures_iter(&text) {
        let command = caps.get(1).unwrap().as_str();
        let count = u32::from_str(caps.get(2).unwrap().as_str()).expect("Parse failed");
        if command.eq("forward") {
            position += count;
            depth += aim * count;
        } else if command.eq("up") {
            aim -= count;
        } else if command.eq("down") {
            aim += count;
        }
    }
    println!["depth: {}", depth];
    println!["position: {}", position];
    println!["aim: {}", aim];
    println!["answer = depth * position = {}", depth * position];
}
