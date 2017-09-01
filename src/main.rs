extern crate rand;
use rand::{thread_rng, Rng};
use std::fmt;

struct Cell {
    alive: bool,
    next: bool,
}

struct Set {
    x: u32,
    y: u32,
    pub alive_c: char,
    pub dead_c: char,
    cells: Vec<Cell>,
}

impl Cell {
    fn new() -> Cell {
        Cell { alive: false, next: false}
    }
    fn queue(&mut self, status: bool) {
        self.next = status;
    }
    fn update(&mut self) {
        self.alive = self.next;
    }
    fn set(&mut self, status: bool) {
        self.alive = status;
    }
}

impl Set {
    pub fn new(x_dim: u32, y_dim: u32) -> Set {
        let t = Vec::<Cell>::with_capacity((x_dim * y_dim) as usize);
        Set {
            x: x_dim,
            y: y_dim,
            alive_c: 'o',
            dead_c: ' ',
            cells: t,
        }
    }

    fn initialize(&mut self) {
        let mut rng = thread_rng();
        let mut i = 0u32;
        while i < self.x*self.y {
            let mut c = Cell::new();
            c.set(rng.gen_weighted_bool(3));
            self.cells.push(c);
            i += 1;
        }
    }

    fn sum_neighbors(&self, index: usize) -> u32 {
       let mut sum = 0u32;
       let length = self.cells.len();
       let y = self.y as usize;
       sum += if self.cells[index-y-1 + length % length].alive {1} else {0};
       sum += if self.cells[index-y   + length % length].alive {1} else {0};
       sum += if self.cells[index-y+1 + length % length].alive {1} else {0};
       sum += if self.cells[index-1   + length % length].alive {1} else {0};
       sum += if self.cells[index+1   + length % length].alive {1} else {0};
       sum += if self.cells[index+y-1 + length % length].alive {1} else {0};
       sum += if self.cells[index+y   + length % length].alive {1} else {0};
       sum += if self.cells[index+y+1 + length % length].alive {1} else {0};
       sum
    }
    fn update(&mut self) {
        let mut counter = 0;
        let length = self.cells.len();
        while counter < length {
            let neighbors = self.sum_neighbors(counter);
            let c = &mut self.cells[counter];
            if c.alive && (neighbors == 2 || neighbors == 3) {
                c.queue(true);
            } else if !c.alive && neighbors == 3 {
                c.queue(true);
            } else {
                c.queue(false);
            }
            counter += 1;
        }
        //actually update values
        for c in self.cells.iter_mut() {
            c.update();
        }
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        let mut counter = 0u32;
        for i in self.cells.iter() {
            s.push(if i.alive { self.alive_c } else { self.dead_c });
            counter += 1;
            if counter == self.x {
                counter = 0;
                s.push('\n');
            }
        }

        write!(f, "{}", s)
    }
}

fn main() {
    let mut game = Set::new(20, 20);
    game.initialize();

    loop {
        println!("{}", game);
        game.update();
    }
}
