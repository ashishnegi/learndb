use sqliters::statement;

pub fn process_sql_command(command : &str)
{
    let result = statement::prepare_statement(command);
    match result {
        Ok(statement) => {
            match statement {
                statement::Statement::Insert => {
                    println!("Executed Insert :P")
                },
                statement::Statement::Select => {
                    println!("Executed Select :P")
                }
            }
        },
        Err(msg) => println!("Failure: {} for command '{}'", msg, command)
    }
}
