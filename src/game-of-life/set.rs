mod cell;
use rand::Rng;

struct set {
    x: u32,
    y: u32,
    aliveC: char,
    deadC: char,
    cells: Vec<cell>,
}

impl set {
    fn new(x_dim: u32, y_dim: u32) -> set {
        set {
            x: x_dim,
            y: y_dim,
            aliveC: 'o',
            deadC: ' ',
            cells: Vec::with_capacity(x_dim * y_dim),
        }
    }

    fn initialize(&mut self) {
        let mut rng = rand::thread_rng();
        for c in self.cells.iter_mut() {
            c.set(rng.gen_weighted_bool(3));
        }
    }

    priv fn sum_neighbors(&self, index: u32) -> u32 {
        
    }


    fn update(&mut self) {
        let mut counter = 0u32;
        for c in self.cells.iter_mut() {
            let neighbors = sum_neighbors(counter);
            match c {
                x if x.alive && (neighbors == 2 || neighbors == 3) => x.queue(true),
                x if !x.alive && neighbors == 3 => x.queue(true),
                x => x.queue(false),
            }
            counter += 1;
        }

        //actually update values
        for c in self.cells.iter_mut() {
            c.update();
        }
    }
}
