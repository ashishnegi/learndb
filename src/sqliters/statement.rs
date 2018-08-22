use std::fmt::{self, Write};
use sqliters::consts;

#[derive(Debug)]
pub enum Statement {
    Insert(InsertStatement),
    Select
}

#[derive(Debug, Default)]
pub struct InsertStatement {
    id: i32,
    username: [u8; consts::USERNAME_SIZE],
    email: [u8; consts::EMAIL_SIZE]
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
        .map_err(|_e| format!("First argument '{}' should be id : integer", command_splits[1]))?;

    let username = command_splits[2];
    let username_bytes = username.as_bytes();
    if username_bytes.len() > consts::USERNAME_SIZE {
        return Err(format!("Username '{}' can be maximum of {} bytes", username, consts::USERNAME_SIZE))
    }

    let email = command_splits[3];
    let email_bytes = email.as_bytes();
    if email_bytes.len() > consts::EMAIL_SIZE {
        return Err(format!("Email '{}' can be maximum of {} bytes", email, consts::EMAIL_SIZE))
    }

    let mut insert: InsertStatement = Default::default();
    insert.id = user_id;
    insert.username[..username_bytes.len()].copy_from_slice(username_bytes);
    insert.email[..email_bytes.len()].copy_from_slice(email_bytes);

    Ok(Statement::Insert(insert))
}

fn prepare_select_statement(command_splits: Vec<&str>) -> Result<Statement, String>
{
    if command_splits.len() != 1 {
        return Err(String::from("Bad insert command : Length != 1"))
    }
    return Ok(Statement::Select)
}

pub fn serialize_row(insert: InsertStatement) -> Result<Vec<u8>, String>
{
    use std::mem::transmute;

    let mut serialized = Vec::<u8>::new();
    let id_bytes: [u8; consts::ID_SIZE] = unsafe { transmute(insert.id.to_be()) };
    serialized.extend_from_slice(&id_bytes);
    serialized.extend_from_slice(&insert.username);
    serialized.extend_from_slice(&insert.email);
    if serialized.len() != consts::INSERT_STATEMENT_SIZE {
        return Err(format!("serialized size is not {}", consts::INSERT_STATEMENT_SIZE))
    }

    Ok(serialized)
}

pub fn deserialize_row(deserialized: Vec<u8>) -> Result<InsertStatement, String>
{
    use std::mem::transmute;
    if deserialized.len() != consts::INSERT_STATEMENT_SIZE {
        return Err(format!("deserialized size is not {}", consts::INSERT_STATEMENT_SIZE))
    }

    let mut insert: InsertStatement = Default::default();

    let mut id_bytes: [u8; consts::ID_SIZE] = Default::default();
    id_bytes.copy_from_slice(&deserialized[0..consts::ID_SIZE]);
    insert.id = unsafe { transmute::<[u8;4], i32>(id_bytes) }.to_be();

    insert.username.copy_from_slice(&deserialized[consts::USERNAME_OFFSET..consts::EMAIL_OFFSET]);
    insert.email.copy_from_slice(&deserialized[consts::EMAIL_OFFSET..]);

    Ok(insert)
}

impl fmt::Display for InsertStatement {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_char('{')?;
        fmt.write_str(" InsertStatement : Id: '")?;
        fmt.write_str(self.id.to_string().as_str())?;
        fmt.write_str("', username: '")?;
        fmt.write_str(String::from_utf8(self.username.to_vec()).expect("Unable to get username from utf8").as_str())?;
        fmt.write_str("', email: '")?;
        fmt.write_str(String::from_utf8(self.email.to_vec()).expect("Unable to get email from utf8").as_str())?;
        fmt.write_str("' }")?;
        Ok(())
    }
}

impl InsertStatement {
    pub fn id(&self) -> i32 {
        self.id
    }
}