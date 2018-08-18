use sqliters::statement;

pub fn process_sql_command(command : &str)
{
    let result = statement::prepare_statement(command);
    match result {
        Ok(statement) => {
            execute_statement(statement)
        },
        Err(msg) => println!("Failure: {} for command '{}'", msg, command)
    }
}

fn execute_statement(statement: statement::Statement)
{
    match statement {
        statement::Statement::Insert(_insert_statement) => {
            println!("Executed Insert :P")
        },
        statement::Statement::Select => {
            println!("Executed Select :P")
        }
    }
}
