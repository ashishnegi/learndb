use sqliters::statement;
use std::iter;

const PAGE_SIZE: usize = 2046;
const TABLE_MAX_PAGES: usize = 100;
const ROW_SIZE: usize = statement::INSERT_STATEMENT_SIZE;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

pub struct Table {
    pages: Vec<Vec<u8>>,
    num_rows: usize
}

impl Table {
    pub fn new() -> Self {
        Table {
            pages: iter::repeat(vec![]).take(TABLE_MAX_PAGES).collect(),
            num_rows: 0
        }
    }

    pub fn row_slot(&mut self, row_num: usize) -> Result<&mut [u8], String> {
        if row_num >= TABLE_MAX_ROWS {
            return Err(format!("{} row is out of space allocated to table {}", row_num, TABLE_MAX_ROWS))
        }

        let page_num = row_num / ROWS_PER_PAGE;
        let row_offset = (row_num % ROWS_PER_PAGE) * ROW_SIZE;
        if self.pages[page_num].len() == 0 {
            self.pages[page_num] = vec![0; PAGE_SIZE];
        }

        return Ok(&mut (self.pages[page_num][row_offset..row_offset + ROW_SIZE]))
    }

    pub fn add_row(&mut self, data: Vec<u8>) -> Result<(), String> {
        if data.len() != ROW_SIZE {
            return Err(format!("Can't store a data of size {} != {}", data.len(), ROW_SIZE))
        }

        {
            let num_rows = self.num_rows;
            let next_slot = self.row_slot(num_rows)?;
            for pos in 0..data.len() {
                next_slot[pos] = data[pos];
            }
        }

        self.num_rows += 1;

        Ok(())
    }

    pub fn num_rows(& self) -> usize {
        self.num_rows
    }
}
