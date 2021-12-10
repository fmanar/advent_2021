use regex::Regex;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str::FromStr;

fn main() {
    let lines = read_file();
    println!("{} lines", lines.len());

    // determine size of image
    let mut max: i32 = 0;
    for line in &lines {
        if line.start.0 > max {
            max = line.start.0;
        }
        if line.start.1 > max {
            max = line.start.1;
        }
        if line.end.0 > max {
            max = line.end.0;
        }
        if line.end.1 > max {
            max = line.end.1;
        }
    }
    println!("max val {}", max);

    // increment to get max val
    max += 1;

    // part 1: only horizontal or vertical
    let mut image = Image::new(max as usize, max as usize);
    for line in &lines {
        image.add_line_1(line);
    }
    println!("Part 1: {} overlaps", image.count());

    // part 2: diagonal, horizontal, or vertical
    let mut image = Image::new(max as usize, max as usize);
    for line in &lines {
        image.add_line_2(line);
    }
    println!("Part 2: {} overlaps", image.count());
}

struct Line {
    start: (i32, i32),
    end: (i32, i32),
}


// this function _should_ pass any errors back up the chain
// but I can't get the generics to work
fn read_file() -> Vec<Line> {
    let file = File::open("./input").expect("Read failed");
    let buffer = BufReader::new(file);
    let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").expect("Regex failed");
    let mut result: Vec<Line> = Vec::new();
    for text in buffer.lines().map(|l| l.unwrap()) {
        let cap = re.captures(&text)
                    .expect("Match failed");
        let num: Vec<i32> = cap.iter()
                .skip(1)
                .map(|m| i32::from_str(m.unwrap()
                                        .as_str()).expect("i32 failed"))
                .collect();
        assert!(num.len() == 4);
        let line = Line {
            start: (num[0], num[1]),
            end: (num[2], num[3]),
        };
        result.push(line);
    }
    result
}

struct Image {
    data: Vec<u32>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(width: usize, height: usize) -> Self {
        Image {
            data: vec![0; width * height],
            width,
            height,
        }
    }
    fn add_line_1(&mut self, line: &Line) {
        let di = line.end.0 - line.start.0;
        let dj = line.end.1 - line.start.1;
        let steps;
        if (di != 0) && (dj != 0) {
            return;
        } else if di == 0 {
            steps = dj.abs() + 1;
        } else {
            steps = di.abs() + 1;
        }
        let mut i = line.start.0;
        let mut j = line.start.1;
        for _ in 0 .. steps {
            let c = i as usize * self.width + j as usize;
            self.data[c] += 1;
            i += di.signum();
            j += dj.signum();
        }

    }
    fn add_line_2(&mut self, line: &Line) {
        let di = line.end.0 - line.start.0;
        let dj = line.end.1 - line.start.1;
        let steps;
        if (di != 0) && (dj != 0) && (di.abs() != dj.abs()) {
            return;
        } else if di == 0 {
            steps = dj.abs() + 1;
        } else {
            steps = di.abs() + 1;
        }
        let mut i = line.start.0;
        let mut j = line.start.1;
        for _ in 0 .. steps {
            let c = (i as usize) * self.width + (j as usize);
            self.data[c] += 1;
            i += di.signum();
            j += dj.signum();
        }

    }
    fn count(&self) -> usize {
        self.data
            .iter()
            .filter(|x| x > &&1)
            .count()
    }
}
