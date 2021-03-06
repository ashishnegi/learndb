use std;
use sqliters::{table, page, consts};

#[derive(Debug)]
pub struct Cursor<'a> {
    table: &'a mut table::Table,
    page_num: u64, // 0..N
    cell_num: u64,
    end_of_table: bool
}

impl<'a> Cursor<'a> {
    pub fn table_start(table: &'a mut table::Table) -> Result<Self, String> {
        let num_pages = table.num_pages();
        let mut page_num = 0;
        let mut cell_num = 0;
        if num_pages != 0 {
            let key_pos = table.find_key_pos(std::i32::MIN)?; // go to last leaf.
            page_num = key_pos.0;
            cell_num = key_pos.1;
        }

        Ok(Cursor {
            table: table,
            page_num: page_num,
            cell_num: cell_num,
            end_of_table: num_pages == 0
        })
    }

    pub fn table_find(table: &'a mut table::Table, key: i32) -> Result<Self, String> {
        let num_pages = table.num_pages();
        let mut page_num = 0;
        let mut cell_num = 0;
        if num_pages != 0 {
            let key_pos = table.find_key_pos(key)?;
            page_num = key_pos.0;
            cell_num = key_pos.1;
        }

        // println!("For key {}, num_pages {}, cell_num {}", key, num_pages, cell_num);

        Ok(Cursor {
            table: table,
            page_num: page_num, // page_num is index.
            cell_num: cell_num,
            end_of_table: true // we don't want to advance ahead.
        })
    }

    pub fn cursor_value(&mut self) -> Result<&mut[u8], String> {
        self.cell_slot()
            .map(|c| &mut c[consts::VALUE_OFFSET..])
    }

    pub fn advance_cursor(&mut self) -> Result<(), String> {
        if !self.end_of_table {
            self.cell_num += 1;
            loop {
                let page = self.table.get_page(self.page_num as usize)?;
                if self.cell_num >= page.num_cells() as u64 {
                    let old_page_num = self.page_num;
                    self.page_num = page.next_sibling_num();
                    // value 0 means that no next_sibling.
                    if self.page_num == 0 {
                        self.end_of_table = true;
                        return Ok(())
                    }

                    assert!(old_page_num != self.page_num, "{} referes to next_sibling {}", old_page_num, self.page_num);
                    // otherwise, since cells are sorted, start from next page's 0th cell_num
                    self.cell_num = 0;
                    continue;
                }

                break;
            }
        }
        Ok(())
    }

    pub fn serialize_row_add(&mut self, data: Vec<u8>) -> Result<(), String> {
        let key = page::deserialize_key(&data[0 .. consts::KEY_SIZE]);

        if self.table.get_page(self.page_num as usize)?.num_cells() >= consts::CELLS_PER_PAGE as u64 {
            // split this page.
            self.table.split_page(self.page_num)?;
            let key_pos = self.table.find_key_pos(key)?;
            self.page_num = key_pos.0;
            self.cell_num = key_pos.1;
        }

        self.add_row(key, data)?;
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

        return Ok(&mut page.get_data()[row_offset .. row_offset + consts::CELL_SIZE])
    }

    fn add_row(&mut self, key: i32, data: Vec<u8>) -> Result<(), String> {
        if data.len() != consts::ROW_SIZE {
            return Err(format!("Can't store a data of size {} != {}", data.len(), consts::ROW_SIZE))
        }

        let page = self.table.get_page(self.page_num as usize)?;

        // println!("adding row : key: {}, page_num {}, cell_num {}, page {:?}", key, self.page_num, self.cell_num, page);

        if key == page.get_key_at(self.cell_num) {
            return Err(format!("Can not insert duplicate keys {}; Already present at pos: {}", key, self.cell_num))
        }

        page.add_data(self.cell_num, &data)?;

        Ok(())
    }
}