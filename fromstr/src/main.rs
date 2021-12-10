fn main() {
    let val = u32::from_str_radix("00100101", 2).expect("parse failed");
    for i in 0..8 {
        println!("{}", bin_digit(val, i));
    }
}

fn bin_digit(num: u32, digit: usize) -> u32 {
    (num >> digit) & 1
}
