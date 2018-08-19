use sqliters::table;

#[derive(Debug)]
pub struct Cursor<'a> {
    table: &'a mut table::Table,
    row_num: usize, // 0..N
    end_of_table: bool
}

impl<'a> Cursor<'a> {
    pub fn table_start(table: &'a mut table::Table) -> Self {
        let num_rows = table.num_rows();
        Cursor {
            table: table,
            row_num: 0,
            end_of_table: num_rows == 0
        }
    }

    pub fn table_end(table: &'a mut table::Table) -> Self {
        let num_rows = table.num_rows();
        Cursor {
            table: table,
            row_num: num_rows,
            end_of_table: true
        }
    }

    pub fn cursor_value(&mut self) -> Result<&mut[u8], String> {
        self.table.row_slot(self.row_num)
    }

    pub fn advance_cursor(&mut self) {
        if !self.end_of_table {
            self.row_num += 1;
            if self.table.num_rows() >= self.row_num {
                self.end_of_table = true;
            }
        }
    }

    pub fn serialize_row_add(&mut self, data: Vec<u8>) -> Result<(), String> {
        self.table.add_row(data)?;
        self.advance_cursor();
        Ok(())
    }

    pub fn end_of_table(&self) -> bool {
        self.end_of_table
    }
}