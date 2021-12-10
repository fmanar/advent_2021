use std::fs;

fn main() {
    let strings = fs::read_to_string("./input").expect("Read failed");
    let mut iter = strings.split_whitespace().peekable();
    let num_digits = match iter.peek() {
        Some(string) => string.len(),
        None => panic!["Nothing in file"]
    };
    println!["{} digits", num_digits];

    let mut counts = vec![0; num_digits];
    let mut num_entry = 0;
    for string in iter {
        for (i, ch) in string.chars().enumerate() {
            if ch == '1' {
                counts[i] += 1;
            } else if ch == '0' {
                counts[i] += 0;
            } else {
                panic!["Bad character"];
            }
        }
        num_entry += 1;
    }
    println!["{} entries", num_entry];
    println!["{:?} counts", counts];

    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();
    for count in counts.into_iter() {
        if count > num_entry / 2 {
            gamma.push(1);
            epsilon.push(0);
        } else {
            gamma.push(0);
            epsilon.push(1);
        }
    }
    println!["{:?} gamma", gamma];
    println!["{:?} epsilon", epsilon];

    let gamma = to_decimal(gamma);
    let epsilon = to_decimal(epsilon);
    println!["{} gamma", gamma];
    println!["{} epsiolon", epsilon];
    println!["{} result = gamma * epsilon", gamma * epsilon];
}

fn to_decimal(binary: Vec<u32>) -> u32 {
    let mut decimal = 0;
    for (pow, digit) in binary.into_iter().rev().enumerate() {
        decimal += digit * 2_u32.pow(pow as u32);
    }
    decimal
}
