use regex::Regex;
use itertools::Itertools;

fn main() {
    let data = read_input("input").expect("Read fail");
    println!("{} entries", data.len());
    part_one(&data);
    part_two(&data);
}

struct Display {
    note: Vec<String>,
    read: Vec<String>,
}

fn read_input(file: &str) -> std::io::Result<Vec<Display>> {
    let text = std::fs::read_to_string(file)?;
    let re = Regex::new(r"([a-z ]+)\|([a-z ]+)").expect("Regex creation fail");
    let mut result = Vec::new();
    for cap in re.captures_iter(&text) {
        let note: Vec<String> = cap[1]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
        let read: Vec<String> = cap[2]
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
        result.push(Display {note, read});
    }
    Ok(result)
}

fn part_one(data: &Vec<Display>) {
    // count all the
    // 1: 2 chars
    // 4: 4 chars
    // 7: 3 chars
    // 8: 7 chars
    // in read-outs
    let mut count = 0;
    for d in data {
        for r in &d.read {
            let n = r.chars().count();
            if n == 2 || n == 3 || n == 4 || n == 7 {
                count += 1;
            }
        }
    }
    println!("Part 1: {}", count);
}

fn part_two(data: &Vec<Display>) {
    // decode and add
    let mut total = 0;
    for disp in data {
        let key = solve_note(&disp.note);
        let delta = solve_read(&key, &disp.read);
        total += delta;
        println!("{:?}: {}", disp.read, delta);
    }
    println!("Part 2: {}", total);
}

/// determine answer key
/// 0: 6 chars, fails 6 and 9 check
/// 1: 2 chars
/// 2: 5 chars, fails 3 and 5 check
/// 3: 5 chars, 2c from 1
/// 4: 4 chars
/// 5: 5 chars, 3c from 4
/// 6: 6 chars, 1c from 1
/// 7: 3 chars
/// 8: 7 chars
/// 9: 6 chars, 4c from 4
///
/// 2c: 1
/// 3c: 7
/// 4c: 4
/// 5c: 2, 3, 5
/// 6c: 0, 6, 9
/// 7c: 8
fn solve_note(note: &Vec<String>) -> Vec<String> {
    let mut key = vec!["".to_string(); 10];

    // find 1, 4, 7, 8
    for s in note {
        let n = s.chars().count();
        match n {
            2 => key[1] = s.clone(),
            3 => key[7] = s.clone(),
            4 => key[4] = s.clone(),
            7 => key[8] = s.clone(),
            _ => (),
        }
    }

    // decode the rest
    for s in note {
        let n = s.chars().count();
        if n == 5 {
            if count_common(s, &key[1]) == 2 {
                key[3] = s.clone();
            } else if count_common(s, &key[4]) == 3 {
                key[5] = s.clone();
            } else {
                key[2] = s.clone();
            }
        } else if n == 6 {
            if count_common(s, &key[1]) == 1 {
                key[6] = s.clone();
            } else if count_common(s, &key[4]) == 4 {
                key[9] = s.clone();
            } else {
                key[0] = s.clone();
            }

        }
    }

    // println!("Key:");
    // for (i, k) in key.iter().enumerate() {
    //     println!("{} {}", i, k);
    // }
    key
}

/// translate readout
fn solve_read(key: &Vec<String>, read: &Vec<String>) -> usize {
    let mut value = 0;
    for (i, r) in read.iter().rev().enumerate() {
        let d = solve_digit(r, key);
        value += d * 10_usize.pow(i as u32);
        // println!("{}: {}", r, d);
    }
    value
}

/// translate a single digit
fn solve_digit(digit: &str, key: &Vec<String>) -> usize {
    for (i, k) in key.iter().enumerate() {
        let n_digit = digit.chars().count();
        let n_key = k.chars().count();
        let c = count_common(digit, k);
        if n_digit == n_key && n_digit == c {
            return i;
        }
    }
    panic!["digit not in key"]
}

/// count common letters
fn count_common(s1: &str, s2: &str) -> usize {
    let mut iter1 = s1.chars().sorted();
    let mut iter2 = s2.chars().sorted();
    let mut opt1 = iter1.next();
    let mut opt2 = iter2.next();
    let mut count = 0;
    loop {
        if let Some(c1) = opt1 {
            if let Some(c2) = opt2 {
                if c1 == c2 {
                    count += 1;
                    opt1 = iter1.next();
                    opt2 = iter2.next();
                } else if c1 < c2 {
                    opt1 = iter1.next();
                } else if c2 < c1 {
                    opt2 = iter2.next();
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    count
}

#[test]
fn test_count_common() {
    assert_eq!(count_common("field", "aditi"), 2);
}
