use std::process;

pub fn process_meta_command(command: &str)
{
    match command {
        ".exit" => process::exit(0),
        _ => {
            println!("Unknown command '{}'", command)
        }
    }
}