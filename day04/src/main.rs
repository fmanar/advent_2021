// I defaulted to a looping instead of anything fancy
use std::fs;
use std::str::FromStr;

fn main() {
    // parse file
    let file: String = fs::read_to_string("./input").expect("Read failed");
    let mut chunks = file.split("\n\n");

    let balls: Vec<u32> = chunks.next().expect("No chunks")
            .split(',')
            .map(|s| u32::from_str(s).expect("Parse failed"))
            .collect();

    let mut boards = Vec::new();
    for chunk in chunks {
        let values = chunk.split_whitespace()
                .map(|s| u32::from_str(s).expect("Parse failed"))
                .collect();
        let board = Board::new(values);
        boards.push(board);
    }

    // call balls

    // use this for first part (first winner)
    // 'outer: for ball in balls {
    //     for board in boards.iter_mut() {
    //         board.mark(ball);
    //         if board.is_winner() {
    //             println!("{}", board.score(ball));
    //             break 'outer;
    //         }
    //     }
    // }

    // use this for second part (last winner)
    for ball in balls {
        println!("ball {:2}, {} remain", ball, boards.len());

        // mark boards
        for board in boards.iter_mut() {
            board.mark(ball);
        }

        // see if the last board won
        if boards.len() == 1 && boards[0].is_winner() {
            println!("{}", boards[0].score(ball));
            break;
        }

        // filter winning boards
        boards = boards.into_iter()
            .filter(|b| !b.is_winner())
            .collect();
    }
}

struct Spot {
    value: u32,
    called: bool,
}

struct Board {
    grid: Vec<Spot>
}

impl Board {
    fn new(values: Vec<u32>) -> Board {
        assert!(values.len() == 25);
        let mut board = Board { grid: Vec::new() };
        for value in values.into_iter() {
            board.grid.push(Spot { value: value, called: false });
        }
        board
    }
    fn mark(&mut self, value: u32) {
        for spot in self.grid.iter_mut() {
            if spot.value == value {
                spot.called = true;
                break;
            }
        }
    }
    fn is_winner(&self) -> bool {
        // check cols
        for col in 0 .. 5 {
            let mut complete = true;
            for row in 0 .. 5 {
                let c = ij_to_c(row, col, 5);
                if !self.grid[c].called {
                    complete = false;
                    break;
                }
            }
            if complete {
                return true;
            }
        }

        // check rows
        for row in 0 .. 5 {
            let mut complete = true;
            for col in 0 .. 5 {
                let c = ij_to_c(row, col, 5);
                if !self.grid[c].called {
                    complete = false;
                    break;
                }
            }
            if complete {
                return true;
            }
        }

        // no luck
        return false
    }
    fn score(&self, last: u32) -> u32 {
        self.grid.iter()
            .filter(|x| !x.called)
            .map(|x| x.value)
            .sum::<u32>() * last
    }
}

fn ij_to_c(i: usize, j: usize, ncols: usize) -> usize {
    i * ncols + j
}
