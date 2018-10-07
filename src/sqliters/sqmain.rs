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
        None => Err(String::from("command expected."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqliters::{consts, statement};
    use std::{fs, path::Path};

    #[test]
    fn test_1_insert_select()
    {
        let db_filename = "test1.db";
        test_setup(db_filename);

        let mut table = table::Table::new(db_filename).expect("Unable to create/open db file.");
        let commands = ["insert 1 ashishnegi abc@abc.com", "select"];
        let mut context = context::Context::new(Box::new(AssertSelectOutFn::new(1)));

        for command in commands.iter() {
            process_command(&mut context, &mut table, command).expect(format!("Failed at command '{}'", command).as_str());
        }
        table.delete_db().expect("Unable to delete test db");
    }

    #[test]
    fn test_1_page_insert_select()
    {
        let db_filename = "test_1_page.db";
        test_setup(db_filename);

        let mut table = table::Table::new(db_filename).expect("Unable to create/open db file.");
        let commands: Vec<String> =  (1 .. consts::CELLS_PER_PAGE)
            .map(|s| format!("insert {} ashishnegi abc@abc.com", s))
            .collect::<Vec<String>>();
        let mut context = context::Context::new(Box::new(AssertSelectOutFn::new(1)));

        for command in commands.iter() {
            process_command(&mut context, &mut table, command).expect(format!("Failed at command '{}'", command).as_str());
        }

        assert!(process_command(&mut context, &mut table, "select").is_ok(), "select should always work");
        table.delete_db().expect("Unable to delete test db");
    }

    #[test]
    fn test_inserts_select()
    {
        let db_filename = "test2.db";
        test_setup(db_filename);

        let mut table = table::Table::new(db_filename).expect("Unable to create/open db file.");
        let mut context = context::Context::new(Box::new(AssertSelectOutFn::new(1)));
        let mut commands: Vec<String> =  (1 .. consts::TABLE_MAX_ROWS)
            .map(|s| format!("insert {} ashishnegi abc@abc.com", s))
            .collect::<Vec<String>>();
        commands.push(String::from("select"));

        for command in commands.iter() {
            process_command(&mut context, &mut table, command)
                .expect(format!("Failed at command '{}' : table : {} \r\n : {:?} ", command, table.print(), table).as_str());
        }
        // make sure that select saw all the rows.
        if let Some(foo) = context.get_out().downcast_ref::<AssertSelectOutFn>() {
            assert!(foo.count() as usize == consts::TABLE_MAX_ROWS,
                "Should be able to see all data written {}", foo.count());
        } else {
            assert!(true, "Failed to get AssertSelectOutFn out of context");
        }
        table.delete_db().expect("Unable to delete test db");
    }

    #[test]
    fn test_max_inserts_select()
    {
        let db_filename = "test3.db";
        test_setup(db_filename);

        let mut table = table::Table::new(db_filename).expect("Unable to create/open db file.");
        let commands: Vec<String> = (1 .. consts::TABLE_MAX_ROWS)
            .map(|s| format!("insert {} ashishnegi abc@abc.com", s))
            .collect::<Vec<String>>();
        let mut context = context::Context::new(Box::new(AssertSelectOutFn::new(1)));

        for command in commands.iter() {
            process_command(&mut context, &mut table, command).expect(format!("Failed at command '{}', table {:?}", command, table).as_str());
        }
        assert!(process_command(&mut context, &mut table, "insert 2 abc abc@bcd.com").is_err(), "should not be able to insert more data");
        assert!(process_command(&mut context, &mut table, "select").is_ok(), "select should always work");
        // make sure that select saw all the rows.
        if let Some(foo) = context.get_out().downcast_ref::<AssertSelectOutFn>() {
            assert!(foo.count() as usize == consts::TABLE_MAX_ROWS,
                "Should be able to see all data written {}", foo.count());
        } else {
            assert!(true, "Failed to get AssertSelectOutFn out of context");
        }
        table.delete_db().expect("Unable to delete test db");
    }

    #[test]
    fn test_random_inserts_sorted_select()
    {
        let db_filename = "test4.db";
        test_setup(db_filename);

        let mut table = table::Table::new(db_filename).expect("Unable to create/open db file.");
        let commands: Vec<String> =  (1 .. consts::TABLE_MAX_ROWS)
            .rev()
            .map(|s| format!("insert {} ashishnegi abc@abc.com", s))
            .collect::<Vec<String>>();
        let mut context = context::Context::new(Box::new(AssertSelectOutFn::new(1)));

        for command in commands.iter() {
            process_command(&mut context, &mut table, command).expect(format!("Failed at command '{}', table {:?}", command, table).as_str());
        }

        table.print();

        assert!(process_command(&mut context, &mut table, "select").is_ok(), "select should always work");

        // make sure that select saw all the rows.
        if let Some(foo) = context.get_out().downcast_ref::<AssertSelectOutFn>() {
            assert!(foo.count() as usize == consts::TABLE_MAX_ROWS,
                "Should be able to see all data written");
        } else {
            assert!(true, "Failed to get AssertSelectOutFn out of context");
        }

        table.delete_db().expect("Unable to delete test db");
    }

    #[test]
    fn test_durability()
    {
        let db_filename = "test_durability.db";
        test_setup(db_filename);

        {
            let mut table = table::Table::new(db_filename).expect("Unable to create/open db file.");
            let commands: Vec<String> =  (1 .. consts::TABLE_MAX_ROWS)
                .rev()
                .map(|s| format!("insert {} ashishnegi abc@abc.com", s))
                .collect::<Vec<String>>();
            let mut context = context::Context::new(Box::new(AssertSelectOutFn::new(1)));

            for command in commands.iter() {
                process_command(&mut context, &mut table, command).expect(format!("Failed at command '{}', table {:?}", command, table).as_str());
            }

            assert!(process_command(&mut context, &mut table, "select").is_ok(), "select should always work");
            if let Some(foo) = context.get_out().downcast_ref::<AssertSelectOutFn>() {
                assert!(foo.count() as usize == consts::TABLE_MAX_ROWS,
                    "Should be able to see all data written");
            } else {
                assert!(false, "Failed to get AssertSelectOutFn out of context");
            }

            table.print();
        }

        {
            let mut table = table::Table::new(db_filename).expect("Unable to create/open db file.");
            let mut context = context::Context::new(Box::new(AssertSelectOutFn::new(1)));

            assert!(process_command(&mut context, &mut table, "select").is_ok(), "select should always work");

            if let Some(foo) = context.get_out().downcast_ref::<AssertSelectOutFn>() {
                assert!(foo.count() as usize == consts::TABLE_MAX_ROWS,
                    "Should be able to see all previous data written after opening file again");
            } else {
                assert!(false, "Failed to get AssertSelectOutFn out of context");
            }

            table.delete_db().expect("Unable to delete test db");
        }
        // add duplicate keys and see that it overrides the old one.
    }

    fn test_setup(db_filename: &str) {
        if Path::new(db_filename).exists() {
            fs::remove_file(db_filename).expect("Should be able to delete db file before starting test");
        }
    }

    pub struct AssertSelectOutFn {
        count: i32
    }

    impl AssertSelectOutFn {
        pub fn new(count: i32) -> Self {
            AssertSelectOutFn{count: count}
        }

        pub fn count(&self) -> i32 {
            self.count
        }
    }

    impl context::OutFn for AssertSelectOutFn {
        fn outfn(&mut self, insert: &statement::InsertStatement) {
            assert!(self.count == insert.id(), "self.count {} == insert.id() {}", self.count, insert.id());
            self.count += 1;
        }
    }
}
