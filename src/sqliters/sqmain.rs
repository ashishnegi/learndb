use std::io::{self, Write};
use sqliters::{metacommands, sqlcommands, table, context};

pub fn sq_main() {
    let mut table = table::Table::new("sqliters.db").expect("Unable to open/create db file.");
    let mut context = context::Context::new(Box::new(context::ConsoleOutFn::new()));

    loop
    {
        print_prompt();

        let mut user_command_input = String::new();
        io::stdin()
            .read_line(&mut user_command_input)
            .expect("Expecting user input");

        let result = process_command(&mut context, &mut table, user_command_input.as_str());

        match result {
            Err(msg) => println!("Error: {}", msg),
            _ => println!("Executed.")
        }
    }
}

fn print_prompt()
{
    print!("db> ");
    io::stdout().flush().expect("failed to flust in print_prompt");
}

fn process_command(context: &mut context::Context, table: &mut table::Table, user_command_input: &str) -> Result<(), String> {
    let user_command = user_command_input.trim();

    match user_command.chars().next() {
        Some('.') => metacommands::process_meta_command(table, user_command),
        Some(_) => sqlcommands::process_sql_command(context, table, user_command),
        None => panic!("Should not come here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqliters::consts;
    use std::{iter, fs};

    #[test]
    fn test_1_insert_select()
    {
        let db_filename = "test1.db";
        fs::remove_file(db_filename);

        let mut table = table::Table::new(db_filename).expect("Unable to create/open db file.");
        let commands = ["insert 1 ashishnegi abc@abc.com", "select"];
        let mut context = context::Context::new(Box::new(context::AssertSelectOutFn::new(1)));

        for command in commands.iter() {
            process_command(&mut context, &mut table, command).expect(format!("Failed at command '{}'", command).as_str());
        }
        table.delete_db().expect("Unable to delete test db");
    }

    #[test]
    fn test_inserts_select()
    {
        let db_filename = "test2.db";
        fs::remove_file(db_filename);

        let mut table = table::Table::new(db_filename).expect("Unable to create/open db file.");
        let mut context = context::Context::new(Box::new(context::AssertSelectOutFn::new(1)));
        let mut commands: Vec<String> =  (1 .. consts::TABLE_MAX_ROWS)
            .map(|s| format!("insert {} ashishnegi abc@abc.com", s))
            .collect::<Vec<String>>();
        commands.push(String::from("select"));

        for command in commands.iter() {
            process_command(&mut context, &mut table, command).expect(format!("Failed at command '{}'", command).as_str());
        }
        table.delete_db().expect("Unable to delete test db");
    }

    #[test]
    fn test_inserts_max_select()
    {
        let db_filename = "test3.db";
        fs::remove_file(db_filename);

        let mut table = table::Table::new(db_filename).expect("Unable to create/open db file.");
        let commands: Vec<&str> = iter::repeat("insert 1 ashishnegi abc@abc.com").take(consts::TABLE_MAX_ROWS).collect::<Vec<&str>>();
        let mut context = context::Context::new(Box::new(context::ConsoleOutFn::new()));

        for command in commands.iter() {
            process_command(&mut context, &mut table, command).expect(format!("Failed at command '{}', table {:?}", command, table).as_str());
        }
        assert!(process_command(&mut context, &mut table, "insert 2 abc abc@bcd.com").is_err(), "should not be able to insert more data");
        assert!(process_command(&mut context, &mut table, "select").is_ok(), "select should always work");
        table.delete_db().expect("Unable to delete test db");
    }
}
