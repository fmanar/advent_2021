// goal: turn this into a generic
//
// a couple custom iterators would have made the functions cleaner

pub struct Matrix {
    data: Vec<u32>,
    shape: (usize, usize),
}

impl Matrix {
    pub fn new(data: Vec<u32>, shape: (usize, usize)) -> Self {
        Matrix { data, shape }
    }
    pub fn get(&self, i: usize, j: usize) -> u32 {
        let c = i * self.shape.1 + j;
        self.data[c]
    }
    pub fn get_shape(&self) -> (usize, usize) {
        self.shape
    }
    pub fn print(&self) {
        println!("Length: {}", self.data.len());
        for c in 0..(self.shape.0 * self.shape.1) {
            print!("{}", self.data[c]);
            if (c + 1) % self.shape.1 == 0 {
                print!("\n");
            }
        }
    }
    pub fn is_low_point(&self, i: usize, j:usize) -> bool {
        let center = self.get(i, j);
        if i > 0 && self.get(i - 1, j) <= center {
            false
        } else if i < self.shape.0 - 1 && self.get(i + 1, j) <= center {
            false
        } else if j > 0 && self.get(i, j - 1) <= center {
            false
        } else if j < self.shape.1 - 1 && self.get(i, j + 1) <= center {
            false
        } else {
            true
        }
    }
}

pub struct Loc(usize, usize);
