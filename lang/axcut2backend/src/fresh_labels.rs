static mut COUNTER: usize = 0;

pub fn fresh_label() -> usize {
    unsafe {
        COUNTER += 1;
        COUNTER
    }
}
