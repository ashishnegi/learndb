use std::io::{self, Write};
use std::process;

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
            Some('.') => process_meta_command(user_command),
            Some(_) => process_sql_command(user_command),
            None => panic!("Should not come here")
        }
    }
}

fn print_prompt()
{
    print!("db> ");
    io::stdout().flush().expect("failed to flust in print_prompt");
}

fn process_sql_command(command : &str)
{
    match command {
        _ => println!("Unknown command '{}'", command)
    }
}

fn process_meta_command(command: &str)
{
    match command {
        ".exit" => process::exit(0),
        _ => {
            println!("Unknown command '{}'", command)
        }
    }
}