use std::process;
use sqliters::{table, page};

pub fn process_meta_command(table: &mut table::Table, command: &str) -> Result<(), String>
{
    match command {
        ".exit" => {
            table.close_db()?;
            process::exit(0)
        },
        ".btree" => {
            page::print_leaf_node(table.get_page(0)?);
            Ok(())
        },
        _ => {
            Err(format!("Unknown command '{}'", command))
        }
    }
}
