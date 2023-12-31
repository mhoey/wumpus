use std::io::stdin;
use crate::ioadapter::ioadapter::IoAdapter;

pub struct Console;

impl IoAdapter for Console {
    fn write(&self, text:&str) {
        println!("{}",text);
    }

    fn read_text(&self) -> String {
        let mut input_text = String::new();
        stdin().read_line(&mut input_text).unwrap();
        return input_text;
    }

    fn read_number(&self) -> u8 {
        let mut input_text = String::new();
        stdin().read_line(&mut input_text).unwrap();
        let number = match input_text.trim().parse::<u8>() {
            Ok(number) => number,
            Err(_) => {
                println!("Expected number (0-255)");
                0
            }
        };
        return number;
    }
}