pub trait IoAdapter {
    fn write(&self, text: &str);
    fn read_text(&self) -> String;
    fn read_number(&self) -> u8;
}