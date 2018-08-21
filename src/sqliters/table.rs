use sqliters::{page, pager, consts};

#[derive(Debug)]
pub struct Table {
    pager: pager::Pager,
    root_page_num: u64,
    num_rows: u64
}

impl Table {
    pub fn new(db_filepath: &str) -> Result<Self, String> {
        let mut pager = pager::Pager::new(consts::PAGE_SIZE, consts::TABLE_MAX_PAGES, db_filepath)?;
        let num_pages = pager.num_db_pages();
        let mut num_rows = 0;
        if num_pages != 0 {
            let page0 = pager.get_page(0)?;
            num_rows = page::get_num_cells(page0);
        }

        Ok(Table {
            pager: pager,
            root_page_num: 0,
            num_rows: num_rows
        })
    }

    pub fn get_page(&mut self, page_num: usize) -> Result<&mut Vec<u8>, String> {
        self.pager.get_page(page_num)
    }

    pub fn delete_db(&mut self) -> Result<(), String> {
        self.pager.delete_db_file()
    }

    pub fn close_db(&mut self) -> Result<(), String> {
        for page_num in 0..self.num_pages() {
            self.pager.flush_page(page_num as usize)?;
        }

        self.pager.close_db()
    }

    pub fn num_pages(&mut self) -> u64 {
        self.pager.num_pages()
    }
}

impl Drop for Table {
    fn drop(&mut self) {
        self.close_db().expect("Unable to close db.")
    }
}
