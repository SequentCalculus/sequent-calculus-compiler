use std::cell::Cell;

struct Budget {
    left: Cell<i64>
}

impl Budget {
    fn new(init: i64) -> Budget {
        Budget{ left: Cell::new(init) }
    }

    fn decr(&self) {
        self.left.replace(self.left.get() - 1);
    }
}