use std::io;
use std::io::Write;

pub struct InputBuffer {
    pub buffer: String,
    pub length: usize,
}

impl InputBuffer {
    pub fn new() -> InputBuffer {
        InputBuffer {
            buffer: String::new(),
            length: 0,
        }
    }

    pub fn print_prompt(&self) {
        print!("rust_db> ");
        match io::stdout().flush() {
            Ok(_) => print!(""),
            Err(error) => println!("{}", error),
        }
    }

    pub fn read_input(&mut self) -> Result<(), io::Error> {
        io::stdin().read_line(&mut self.buffer)?;
        self.length = self.buffer.len();
        Ok(())
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.length = 0;
    }
}