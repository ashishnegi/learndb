use sqliters::{table, page, consts};

#[derive(Debug)]
pub struct Cursor<'a> {
    table: &'a mut table::Table,
    page_num: u64, // 0..N
    cell_num: u64,
    end_of_table: bool
}

impl<'a> Cursor<'a> {
    pub fn table_start(table: &'a mut table::Table) -> Self {
        let num_pages = table.num_pages();
        Cursor {
            table: table,
            page_num: 0,
            cell_num: 0,
            end_of_table: num_pages == 0
        }
    }

    pub fn table_end(table: &'a mut table::Table) -> Self {
        let num_pages = table.num_pages();
        let mut page_num = 0;
        let mut num_cells = 0;
        if num_pages != 0 {
            let page = table.get_page(0)
                .expect("cursor : Failed to get page 0 when confirmed to have page 0"); // 0 is root
            num_cells = page::get_num_cells(page);

            page_num = num_pages - 1;
        }

        Cursor {
            table: table,
            page_num: page_num, // page_num is index.
            cell_num: num_cells,
            end_of_table: true
        }
    }

    pub fn cursor_value(&mut self) -> Result<&mut[u8], String> {
        self.cell_slot()
            .map(|c| &mut c[consts::VALUE_OFFSET..])
    }

    pub fn advance_cursor(&mut self) -> Result<(), String> {
        if !self.end_of_table {
            self.cell_num += 1;
            if self.cell_num >= consts::CELLS_PER_PAGE as u64 {
                self.end_of_table = true;
                return Ok(());
            }

            // otherwise read page and find number of cells.
            let page = self.table.get_page(self.page_num as usize)?;
            let num_cells = page::get_num_cells(page);
            if self.cell_num >= num_cells {
                self.end_of_table = true;
            }
        }
        Ok(())
    }

    pub fn serialize_row_add(&mut self, data: Vec<u8>) -> Result<(), String> {
        if self.cell_num >= consts::TABLE_MAX_ROWS as u64 {
            // currently we have only one page..
            return Err(format!("{} row is out of space allocated to table {}", self.cell_num, consts::TABLE_MAX_ROWS))
        }

        self.add_row(data)?;
        self.advance_cursor()
    }

    pub fn end_of_table(&self) -> bool {
        self.end_of_table
    }

    fn cell_slot(&mut self) -> Result<&mut [u8], String> {
        if self.cell_num >= consts::TABLE_MAX_ROWS as u64 {
            // currently we have only one page..
            return Err(format!("{} row is out of space allocated to table {}", self.cell_num, consts::TABLE_MAX_ROWS))
        }

        let row_offset = consts::PAGE_HEADER_SIZE + (self.cell_num as usize * consts::CELL_SIZE);
        let page = self.table.get_page(self.page_num as usize)?;

        return Ok(&mut page[row_offset .. row_offset + consts::CELL_SIZE])
    }

    fn add_row(&mut self, data: Vec<u8>) -> Result<(), String> {
        if data.len() != consts::ROW_SIZE {
            return Err(format!("Can't store a data of size {} != {}", data.len(), consts::ROW_SIZE))
        }

        let page = self.table.get_page(self.page_num as usize)?;
        let key_pos = page::find_new_key_pos(page, page::deserialize_key(&data[0..consts::KEY_SIZE]))?;
        println!("cell_num {}", self.cell_num);
        page::add_data(page, key_pos, &data)?;
        page::increment_cell_count(page);

        Ok(())
    }
}