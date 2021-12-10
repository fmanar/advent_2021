use std::fs;

fn main() {
    let bit_depth = 12;
    let data: Vec<u32> = fs::read_to_string("./input")
                            .expect("Read failed")
                            .split_whitespace()
                            .map(|x| u32::from_str_radix(x, 2)
                                         .expect("Parse failed"))
                            .collect();
    let oxygen;
    let mut d = bit_depth - 1;
    let mut tmp = data.clone();
    loop {
        let mcd = most_common_digit(&tmp, d);
        tmp = tmp.into_iter().filter(|x| get_digit(*x, d) == mcd).collect();
        println!("{:2}: {} ({} left)", d, mcd, tmp.len());
        if tmp.len() == 1 {
            oxygen = tmp[0];
            break;
        }
        d -= 1;
    }
    println!("oxygen {:012b}", oxygen);

    let carbon;
    d = bit_depth - 1;
    tmp = data.clone();
    loop {
        let lcd = least_common_digit(&tmp, d);
        tmp = tmp.into_iter().filter(|x| get_digit(*x, d) == lcd).collect();
        println!("{:2}: {} ({} left)", d, lcd, tmp.len());
        if tmp.len() == 1 {
            carbon = tmp[0];
            break;
        }
        d -= 1;
    }
    println!("carbon {:012b}", carbon);
    println!("solution {}", oxygen * carbon);
}

fn get_digit(num: u32, digit: usize) -> u32 {
    (num >> digit) & 1
}

fn most_common_digit(numbers: &Vec<u32>, digit: usize) -> u32 {
    let mut count = 0;
    for number in numbers {
        count += get_digit(*number, digit);
    }
    if count >= (numbers.len() / 2) as u32 {
        1
    } else {
        0
    }
}

fn least_common_digit(numbers: &Vec<u32>, digit: usize) -> u32 {
    let mcd = most_common_digit(&numbers, digit);
    if mcd == 1 {
        0
    } else {
        1
    }
}
