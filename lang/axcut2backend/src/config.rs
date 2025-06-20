/// Each variable in AxCut corresponds to two temporaries, with `Fst` and `Snd` distinguishing the
/// two.
#[derive(Copy, Clone)]
pub enum TemporaryNumber {
    Fst = 0,
    Snd = 1,
}

/// This trait abstracts a few configurations details for the backend platforms.
pub trait Config<Temporary, Immediate> {
    fn i64_to_immediate(number: i64) -> Immediate;
    /// Scratch register.
    fn temp() -> Temporary;
    /// Pointer to free list. Always points to a free block of memory (even if the free list itself
    /// is empty).
    fn heap() -> Temporary;
    /// Pointer to lazy todo list.
    fn free() -> Temporary;
    /// First return register.
    fn return1() -> Temporary;
    /// Second return register.
    fn return2() -> Temporary;
    /// Length of jump instruction used in jump tables.
    fn jump_length(n: usize) -> Immediate;
}
