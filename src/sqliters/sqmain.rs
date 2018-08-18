use std::io::{self, Write};

pub fn sq_main() {
    loop
    {
        print_prompt();

        let mut user_command = String::new();
        io::stdin()
            .read_line(&mut user_command)
            .expect("Expecting user input");

        match user_command.trim() {
            ".exit" => {
                return ;
            }

            _ => {
                println!("Unknown command : {}", user_command.trim())
            }
        }
    }
}

fn print_prompt()
{
    print!("db> ");
    io::stdout().flush().expect("failed to flust in print_prompt");
}