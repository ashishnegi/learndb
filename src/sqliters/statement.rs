#[derive(Debug)]
pub enum Statement {
    Insert,
    Select
}

pub fn prepare_statement(command: &str) -> Result<Statement, String>
{
    let splits : Vec<&str> = command.split(char::is_whitespace).collect();
    match splits.first()
    {
        Some(&"insert") => Ok(Statement::Insert),
        Some(&"select") => Ok(Statement::Select),
        _ => Err(String::from("Unknown command"))
    }
}
