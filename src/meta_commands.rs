use crate::buffer::InputBuffer;

pub enum MetaCommandResult {
    MetaCommandSuccess,
    MetaCommandUnrecognizedCommand,
}

pub fn do_meta_command(input_buffer: &InputBuffer) -> MetaCommandResult {
    if input_buffer.buffer.trim() == ".exit" {
        return MetaCommandResult::MetaCommandSuccess;
    } else {
        return MetaCommandResult::MetaCommandUnrecognizedCommand;
    }
}