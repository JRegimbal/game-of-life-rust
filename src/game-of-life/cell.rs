struct cell {
    alive: bool,
    next: bool,
}

impl cell {
    fn new() -> cell {
        cell { alive: false, next: false}
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

