use sqliters::{statement, pager};

const PAGE_SIZE: usize = 2046;
const TABLE_MAX_PAGES: usize = 100;
const ROW_SIZE: usize = statement::INSERT_STATEMENT_SIZE;
const ROWS_PER_PAGE: usize = PAGE_SIZE / ROW_SIZE;
pub const TABLE_MAX_ROWS: usize = ROWS_PER_PAGE * TABLE_MAX_PAGES;

#[derive(Debug)]
pub struct Table {
    pager: pager::Pager,
    num_rows: usize
}

impl Table {
    pub fn new(db_filepath: &str) -> Result<Self, String> {
        let pager = pager::Pager::new(PAGE_SIZE, TABLE_MAX_PAGES, db_filepath)?;
        let last_page_size = pager.last_page_size();
        let last_num_rows = last_page_size / (ROW_SIZE as u64);

        if last_page_size % (ROW_SIZE as u64) != 0 {
            // data corruption. // we can even truncate rest of file.
            return Err(format!("Half rows found in file {} : last_page_size {} : page_size {}",
                db_filepath, last_page_size, PAGE_SIZE))
        }

        let num_rows = pager.num_db_full_pages() * (ROWS_PER_PAGE as u64) + last_num_rows;
        Ok(Table {
            pager: pager,
            num_rows: num_rows as usize
        })
    }

    pub fn row_slot(&mut self, row_num: usize) -> Result<&mut [u8], String> {
        if row_num >= TABLE_MAX_ROWS {
            return Err(format!("{} row is out of space allocated to table {}", row_num, TABLE_MAX_ROWS))
        }

        let page_num = row_num / ROWS_PER_PAGE;
        let row_offset = (row_num % ROWS_PER_PAGE) * ROW_SIZE;
        let page = self.pager.get_page(page_num)?;

        return Ok(&mut page[row_offset..row_offset + ROW_SIZE])
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

    pub fn num_rows(&self) -> usize {
        self.num_rows
    }

    pub fn delete_db(&mut self) -> Result<(), String> {
        self.pager.delete_db_file()
    }

    pub fn close_db(&mut self) -> Result<(), String> {
        for page_num in 0..(self.num_rows / ROWS_PER_PAGE) {
            self.pager.flush_page(page_num, PAGE_SIZE)?;
        }

        let num_rows_in_last_page = self.num_rows % ROWS_PER_PAGE;
        if num_rows_in_last_page != 0 {
            // last page is not full
            self.pager.flush_page(self.num_rows / ROWS_PER_PAGE, num_rows_in_last_page * ROW_SIZE)?
        }

        self.pager.close_db()?;

        Ok(())
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        self.close_db().expect("Unable to close db.")
    }
}
