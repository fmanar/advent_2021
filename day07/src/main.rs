// goals:
//  - get file reading into a function with proper generics for
//      array values and Result<>
//  - use generics for function
//  - have function take a slice, not a vec
//  - pass function to optimizer... some how (a closure?)
//      take a type with Fn()-> trait, pg 331
use std::fs;
use std::str::FromStr;

fn main() {
    // read file
    let positions = read_input("./input").expect("Read failed.");
    let min = *(positions.iter().min().unwrap());

    // step through to find minimum
    // a sad brute force option
    let mut prev = min;
    let mut this = prev + 1;
    let mut val_prev = calc_fuel(prev, &positions);
    let mut val_this = calc_fuel(this, &positions);
    while val_this < val_prev {
        prev = this;
        this += 1;
        val_prev = val_this;
        val_this = calc_fuel(this, &positions);
    }
    println!("val_prev {}", val_prev);

    let mut prev = min;
    let mut this = prev + 1;
    let mut val_prev = calc_fuel_2(prev, &positions);
    let mut val_this = calc_fuel_2(this, &positions);
    while val_this < val_prev {
        prev = this;
        this += 1;
        val_prev = val_this;
        val_this = calc_fuel_2(this, &positions);
    }
    println!("val_prev {}", val_prev);
}

/// Read input file
///
/// # Parameters
///
/// - `file`: file name
///
/// # Returns
///
/// Numbers in the file as u64
///
fn read_input(file: &str) -> std::io::Result<Vec<u64>> {
    let array: Vec<u64> = fs::read_to_string("./input")?
        .trim()
        .split(',')
        .map(|s| u64::from_str(s).expect("Parse failed"))
        .collect();
    Ok(array)
}

/// Compute fuel required to move crabs to a target position
///
/// # Parameters
///
/// - `target`: where to move them all
/// - `positions`: crabmarine locations
///
/// # Returns
///
/// How much fuel it will take to get them there
///
fn calc_fuel(target: u64, positions: &Vec<u64>) -> u64 {
    positions.iter()
        .map(|x| {
            if *x >= target {
                *x - target
            } else {
                target - *x
            }
        })
        .sum::<u64>()
}

#[test]
fn test_calc_fuel() {
    let positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    assert_eq!(calc_fuel( 2, &positions), 37);
    assert_eq!(calc_fuel( 1, &positions), 41);
    assert_eq!(calc_fuel( 3, &positions), 39);
    assert_eq!(calc_fuel(10, &positions), 71);
}

fn calc_fuel_2(target: u64, positions: &Vec<u64>) -> u64 {
    positions.iter()
        .map(|x| {
            let delta = if *x >= target {
                    *x - target
                } else {
                    target - *x
                };
            delta * (delta + 1) / 2
        })
        .sum::<u64>()
}

// / Find optimum
// /
// / Use the golden section search to find the optimum given a function.  Since
// / the context is integer values, this function operates on them.
// /
// / # Parameters
// /
// / - `func`: the function to be optimized
// / - `a`: left starting point
// / - `b`: right starting point
// /
// / # Returns
// /
// / The value of func at the minima.
// /
// fn find_optimimum<F>(func: F, a: u64, b: u64) -> u64
//     where F: Fn(u64) -> u64
// {
//     const PHI: f64 = 1.618033988749894;
//     let mut a = a;
//     let mut b = b;

//     loop {
//         x = get_mid(a, b, PHI - 1);
//         left = func(a);
//         mid = func(x);
//         right = func(b);
//     }
// }

// fn has_minima(left: u64, mid: u64, right: u64) -> bool {
//     (min < left) && (mid < right)
// }

// fn get_mid(a: u64, b: u64, ratio: f64) -> u64 {
//     a + ((b - a) as f64 * ratio).round() as u64
// }
