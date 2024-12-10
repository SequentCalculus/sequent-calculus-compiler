#[derive(Copy, Clone)]
pub enum TemporaryNumber {
    Fst = 0,
    Snd = 1,
}

pub trait Config<Temporary, Immediate> {
    fn i64_to_immediate(number: i64) -> Immediate;
    fn temp() -> Temporary;
    fn heap() -> Temporary;
    fn free() -> Temporary;
    fn return1() -> Temporary;
    fn return2() -> Temporary;
    fn jump_length(n: usize) -> Immediate;
}
