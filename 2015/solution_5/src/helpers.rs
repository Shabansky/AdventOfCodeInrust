pub type Byte = u8;

pub trait Rule {
    fn process_char(&mut self, line: &String, index: usize, char: Byte);
    fn passes(&self) -> bool;
    fn reset(&mut self);
}
