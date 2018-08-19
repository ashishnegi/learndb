use std::process;

pub fn process_meta_command(command: &str) -> Result<(), String>
{
    match command {
        ".exit" => process::exit(0),
        _ => {
            Err(format!("Unknown command '{}'", command))
        }
    }
}
