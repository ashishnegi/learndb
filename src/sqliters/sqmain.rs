use std::io::{self, Write};
use sqliters::{metacommands, sqlcommands, table};

pub fn sq_main() {
    let mut table = table::Table::new("sqlitere.db").expect("Unable to open/create db file.");

    loop
    {
        print_prompt();

        let mut user_command_input = String::new();
        io::stdin()
            .read_line(&mut user_command_input)
            .expect("Expecting user input");

        let result = process_command(&mut table, user_command_input.as_str());

        match result {
            Err(msg) => println!("{}", msg),
            _ => println!("Executed.")
        }
    }
}

fn print_prompt()
{
    print!("db> ");
    io::stdout().flush().expect("failed to flust in print_prompt");
}

fn process_command(table: &mut table::Table, user_command_input: &str) -> Result<(), String> {
    let user_command = user_command_input.trim();

    match user_command.chars().next() {
        Some('.') => metacommands::process_meta_command(user_command),
        Some(_) => sqlcommands::process_sql_command(table, user_command),
        None => panic!("Should not come here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter;

    #[test]
    fn test_1_insert_select()
    {
        let mut table = table::Table::new("test1.db").expect("Unable to create/open db file.");
        let commands = ["insert 1 ashishnegi abc@abc.com", "select"];
        for command in commands.iter() {
            process_command(&mut table, command).expect(format!("Failed at command '{}'", command).as_str());
        }
        table.delete_db().expect("Unable to delete test db");
    }

    #[test]
    fn test_inserts_select()
    {
        let mut table = table::Table::new("test2.db").expect("Unable to create/open db file.");
        let mut commands: Vec<&str> = iter::repeat("insert 1 ashishnegi abc@abc.com").take(table::TABLE_MAX_ROWS).collect::<Vec<&str>>();
        commands.push("select");

        for command in commands.iter() {
            process_command(&mut table, command).expect(format!("Failed at command '{}'", command).as_str());
        }
        table.delete_db().expect("Unable to delete test db");
    }

    #[test]
    fn test_inserts_max_select()
    {
        let mut table = table::Table::new("test3.db").expect("Unable to create/open db file.");
        let commands: Vec<&str> = iter::repeat("insert 1 ashishnegi abc@abc.com").take(table::TABLE_MAX_ROWS).collect::<Vec<&str>>();

        for command in commands.iter() {
            process_command(&mut table, command).expect(format!("Failed at command '{}'", command).as_str());
        }

        assert!(process_command(&mut table, "insert 2 abc abc@bcd.com").is_err(), "should not be able to insert more data");
        assert!(process_command(&mut table, "select").is_ok(), "select should always work");
        table.delete_db().expect("Unable to delete test db");
    }
}
