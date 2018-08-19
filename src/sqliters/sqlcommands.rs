use sqliters::{statement, table};

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
    table.add_row(deserialized)
}

fn execute_select_statement(table: &mut table::Table) -> Result<(), String>
{
    for row in 0..table.num_rows() {
        let serialized = table.row_slot(row)?;
        println!("{}", statement::deserialize_row(serialized.to_vec())?)
    }

    Ok(())
}
