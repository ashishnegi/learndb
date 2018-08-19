use std::process;
use sqliters::table;

pub fn process_meta_command(table: &mut table::Table, command: &str) -> Result<(), String>
{
    match command {
        ".exit" => {
            table.close_db()?;
            process::exit(0)
        },
        _ => {
            Err(format!("Unknown command '{}'", command))
        }
    }
}
