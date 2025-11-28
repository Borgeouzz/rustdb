use std::io;
mod buffer;
mod meta_commands;
mod statements;

use crate::buffer::InputBuffer;
use crate::meta_commands::do_meta_command;
use crate::meta_commands::MetaCommandResult;
use crate::statements::{prepare_statement, execute_statement, Statement};
use crate::statements::PrepareResult;

fn main() -> Result<(), io::Error> {
    let mut input_buffer = InputBuffer::new();
    loop {
        input_buffer.print_prompt();
        input_buffer.read_input()?;
        if input_buffer.buffer.starts_with(".") {
            match do_meta_command(&input_buffer) {
                MetaCommandResult::MetaCommandSuccess => break,
                MetaCommandResult::MetaCommandUnrecognizedCommand => {
                    println!("Unrecognized command '{}'", input_buffer.buffer.trim());
                    continue;
                }
            }
        }

        let mut statement = Statement::new();
        match prepare_statement(&input_buffer, &mut statement) {
            PrepareResult::PrepareSuccess => {
                execute_statement(&statement);
                println!("Executed.");
            }
            PrepareResult::PrepareUnrecognizedStatement => {
                println!("Unrecognized keyword at start of '{}'", input_buffer.buffer.trim());
            }
        }
        input_buffer.clear();
    }
    Ok(())
}