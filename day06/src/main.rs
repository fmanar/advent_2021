use std::fs;
use std::str::FromStr;

const NUM_DAYS_1: u32 = 80;
const NUM_DAYS_2: u32 = 256;

fn main() {
    // read file
    let array: Vec<u64> = fs::read_to_string("./input")
            .expect("Read failed")
            .trim()
            .split(',')
            .map(|s| u64::from_str(s).expect("Parse failed"))
            .collect();

    // brute way
    let mut array_1 = array.clone();
    for _ in 0 .. NUM_DAYS_1 {
        iterate(&mut array_1);
    }
    println!("Part 1 ({} days) {}", NUM_DAYS_1, array_1.len());

    // svelte way
    let mut pops = start(&array);
    for _ in 0 .. NUM_DAYS_1 {
        step(&mut pops);
    }
    let num: u64 = pops.iter().sum();
    println!("Part 1 ({} days) {}", NUM_DAYS_1, num);

    let mut pops = start(&array);
    for _ in 0 .. NUM_DAYS_2 {
        step(&mut pops);
    }
    let num: u64 = pops.iter().sum();
    println!("Part 1 ({} days) {}", NUM_DAYS_1, num);

    // oops, ran out of memory
    // let mut array_2 = array.clone();
    // for _ in 0 .. NUM_DAYS_2 {
    //     iterate(&mut array_2);
    // }
    // println!("Part 2 ({} days) {}", NUM_DAYS_2, array_2.len());
}

// brute force method
fn iterate(array: &mut Vec<u64>) {
    let mut new = 0;
    for val in array.iter_mut() {
        if *val == 0 {
            *val = 6;
            new += 1;
        } else {
            *val -= 1;
        }
    }
    for _ in 0 .. new {
        array.push(8);
    }
}

// population method
fn start(array: &Vec<u64>) -> Vec<u64> {
    let mut pops = vec![0; 9];
    for val in array {
        let ind = *val as usize;
        pops[ind] += 1;
    }
    pops
}
fn step(pops: &mut Vec<u64>) {
    let tmp = pops[0];
    for i in 0 .. (pops.len() - 1) {
        pops[i] = pops[i+1];
    }
    pops[6] += tmp;
    pops[8] = tmp;
}


