use crate::buffer::InputBuffer;

pub enum PrepareResult {
    PrepareSuccess,
    PrepareUnrecognizedStatement,
}

pub enum StatementType {
    StatementInsert,
    StatementSelect,
}

pub struct Statement {
    pub statement_type: StatementType,
}

impl Statement {
    pub fn new() -> Statement {
        Statement {
            statement_type: StatementType::StatementSelect,
        }
    }
}

pub fn prepare_statement(input_buffer: &InputBuffer, statement: &mut Statement) -> PrepareResult {
    if input_buffer.buffer.trim() == "insert" {
        statement.statement_type = StatementType::StatementInsert;
    } else if input_buffer.buffer.trim() == "select" {
        statement.statement_type = StatementType::StatementSelect;
    } else {
        return PrepareResult::PrepareUnrecognizedStatement;
    }
    return PrepareResult::PrepareSuccess;
}

pub fn execute_statement(statement: &Statement) {
    match &statement.statement_type {
        StatementType::StatementInsert => {
            println!("This is where we would do an insert.");
        }
        StatementType::StatementSelect => {
            println!("This is where we would do a select.");
        }
    }
}