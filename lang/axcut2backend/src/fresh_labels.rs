static mut COUNTER: usize = 0;

pub fn fresh_label() -> usize {
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}

pub fn set_counter(value: usize) {
    unsafe {
        COUNTER = value;
    }
}
