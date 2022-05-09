// goal: read input filename from prompt, done!
// goal: use logging system for different output levels

mod matrix;
use matrix::Matrix;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} FILE", args[0]);
        eprintln!("Example: {} input", args[0]);
        std::process::exit(1);
    }

    let matrix = read_input(&args[1]).expect("Read failed");

    #[cfg(debug_assertions)]
    matrix.print();

    part_one(&matrix);
}

fn read_input(file: &str) -> std::io::Result<Matrix> {
    println!("File: {}", file);
    let mut data: Vec<u32> = Vec::new();
    let mut shape = (0, 0);

    let text = std::fs::read_to_string(file)?;
    for line in text.split_whitespace() {
        let mut tmp: Vec<u32> = line
            .chars()
            .map(|c| c.to_digit(10).expect("Digit failed"))
            .collect();
        shape.0 += 1;
        shape.1 = tmp.len();
        data.append(&mut tmp);
    }
    println!("Shape: {:?}", matrix.get_shape());
    Ok(Matrix::new(data, shape))
}

fn part_one(matrix: &Matrix) {
    // find the low points
    // tally "risk level" = sum(height + 1)
    let mut risk = 0;
    for i in 0..matrix.get_shape().0 {
        for j in 0..matrix.get_shape().1 {
            if matrix.is_low_point(i, j) {
                #[cfg(debug_assertions)]
                println!("pt ({}, {}) = {}", i, j, matrix.get(i, j));

                risk += matrix.get(i, j) + 1;
            }
        }
    }
    println!("Part one: {}", risk);
}

fn part_two(matrix: &Matrix) {
    // find top 3 largest basins
    // return product of sizes
    // basins are bounded by 9s, ridges discounted by input
    //
    // iterate through low points, for each do recursive search
    // (with a stack) of neighbors counting basin locations as you go
    // stuff basin sizes in a list, sort, pull top three
}
