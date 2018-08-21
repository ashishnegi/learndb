use sqliters::{statement, table, cursor};

pub fn process_sql_command(table: &mut table::Table, command : &str) -> Result<(), String>
{
    let result = statement::prepare_statement(command);
    match result {
        Ok(statement) => {
            execute_statement(table, statement)
        },
        Err(msg) => {
            Err(format!("Failure: {} for command '{}'", msg, command))
        }
    }
}

fn execute_statement(table: &mut table::Table, statement: statement::Statement) -> Result<(), String>
{
    match statement {
        statement::Statement::Insert(insert_statement) => {
            execute_insert_statement(table, insert_statement)
        },
        statement::Statement::Select => {
            execute_select_statement(table)
        }
    }
}

fn execute_insert_statement(table: &mut table::Table, statement: statement::InsertStatement) -> Result<(), String>
{
    let deserialized = statement::serialize_row(statement)?;
    let mut cursor = cursor::Cursor::table_end(table);
    cursor.serialize_row_add(deserialized)
}

fn execute_select_statement(table: &mut table::Table) -> Result<(), String>
{
    let mut cursor = cursor::Cursor::table_start(table);

    while !cursor.end_of_table() {
        {
            let serialized = cursor.cursor_value()?;
            println!("row: {}", statement::deserialize_row(serialized.to_vec())?);
        }
        cursor.advance_cursor();
    }

    Ok(())
}
