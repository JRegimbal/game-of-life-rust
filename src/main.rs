extern crate rand;
use rand::{thread_rng, Rng};
use std::fmt;

#[derive(Copy)]
struct Cell {
    alive: bool,
    next: bool,
}

impl Clone for Cell {
    fn clone(&self) -> Cell { *self }
}

struct Set {
    x: u32,
    y: u32,
    pub alive_c: char,
    pub dead_c: char,
    cells: Vec<Vec<Cell>>,
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
        let t = vec![vec![Cell::new(); x_dim as usize]; y_dim as usize];
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
        for x in self.cells.iter_mut() {
            for y in x {
                y.set(rng.gen_weighted_bool(3));
            }
        }
    }

    fn sum_neighbors(&self, x: i32, y: i32) -> u32 {
       let mut sum = 0u32;
       let mut j = y-1;
       let x_dim = self.x as i32;
       let y_dim = self.y as i32;
       while j <= y+1 {
           let mut i = x-1;
           while i <= x+1 {
               sum += if self.cells[((i+x_dim)%x_dim) as usize][((j+y_dim)%y_dim) as usize].alive {1} else {0};
               i += 1;
           }
           j += 1;
       }
       
       sum -= if self.cells[x as usize][y as usize].alive {1} else {0};
       sum
    }
    fn update(&mut self) {
        for i in 0..self.x-1 {
            for j in 0..self.y-1 {
                let neighbors = self.sum_neighbors(i as i32, j as i32);
                let c = &mut self.cells[i as usize][j as usize];
                if c.alive && (neighbors == 2 || neighbors == 3) {
                    c.queue(true);
                } else if !c.alive && neighbors == 3 {
                    c.queue(true);
                } else {
                    c.queue(false);
                }
            }
        }
        //actually update values
        for a in self.cells.iter_mut() {
            for c in a {
                c.update();
            }
        }
    }
}

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        
        for i in self.cells.iter() {
            for j in i {
                s.push(if j.alive {self.alive_c} else {self.dead_c});
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}

fn main() {
    let x = 20;
    let y = 20;
    let mut game = Set::new(x, y);
    game.initialize();

    loop {
        print!("{}", game);
        game.update();
        std::thread::sleep(std::time::Duration::from_millis(250));
        print!("\x1B[{}A", y);
        print!("\r");
        }
}
