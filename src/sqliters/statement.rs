#[derive(Debug)]
pub enum Statement {
    Insert(InsertStatement),
    Select
}

#[derive(Debug)]
pub struct InsertStatement {
    id: i32,
    username: String,
    email: String
}

pub fn prepare_statement(command: &str) -> Result<Statement, String>
{
    // remove empty string from splits.
    let splits : Vec<&str> = command.split(char::is_whitespace).filter(|c| !c.is_empty()).collect();
    match splits.first()
    {
        Some(&"insert") => prepare_insert_statement(splits),
        Some(&"select") => prepare_select_statement(splits),
        _ => Err(String::from("Unknown command"))
    }
}

fn prepare_insert_statement(command_splits: Vec<&str>) -> Result<Statement, String>
{
    // column 	type
    // id 	    integer
    // username varchar(32)
    // email 	varchar(255)
    // insert 1 ashishnegi thisismyidashish@gmail.com
    if command_splits.len() != 4 {
        return Err(String::from("Bad insert command : Length != 4"))
    }

    let user_id = command_splits[1].parse::<i32>()
        .expect(format!("First argument '{}' should be id : integer", command_splits[1]).as_str());

    let username = command_splits[2];
    let email = command_splits[3];

    return Ok(Statement::Insert(
        InsertStatement{
            id: user_id,
            username: username.to_string(),
            email: email.to_string()
    }))
}

fn prepare_select_statement(command_splits: Vec<&str>) -> Result<Statement, String>
{
    if command_splits.len() != 1 {
        return Err(String::from("Bad insert command : Length != 1"))
    }
    return Ok(Statement::Select)
}
