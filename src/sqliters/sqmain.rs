use std::io::{self, Write};
use sqliters::{metacommands, sqlcommands};

pub fn sq_main() {
    loop
    {
        print_prompt();

        let mut user_command_input = String::new();
        io::stdin()
            .read_line(&mut user_command_input)
            .expect("Expecting user input");

        let user_command = user_command_input.trim();

        match user_command.chars().next() {
            Some('.') => metacommands::process_meta_command(user_command),
            Some(_) => sqlcommands::process_sql_command(user_command),
            None => panic!("Should not come here")
        }
    }
}

fn print_prompt()
{
    print!("db> ");
    io::stdout().flush().expect("failed to flust in print_prompt");
}
