#[derive(Copy, Clone)]
pub enum TemporaryNumber {
    Fst = 0,
    Snd = 1,
}

pub trait Config<Temporary, Immediate> {
    fn i64_to_immediate(&self, number: i64) -> Immediate;
    fn temp(&self) -> Temporary;
    fn heap(&self) -> Temporary;
    fn free(&self) -> Temporary;
    fn return1(&self) -> Temporary;
    fn return2(&self) -> Temporary;
    fn jump_length(&self, n: usize) -> Immediate;
}
