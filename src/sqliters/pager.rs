use std::{iter, fs};
use std::io::{self, Seek, Read, Write};
use std::fs::OpenOptions;
use sqliters::page;

#[derive(Debug)]
pub struct Pager {
    pages: Vec<Vec<u8>>,
    db_file: fs::File,
    db_filepath: String,
    filesize: u64,
    page_size: usize,
    max_pages: usize,
    num_pages: u64
}

impl Pager {
    pub fn new(page_size: usize, max_pages: usize, db_filepath: &str) -> Result<Self, String> {
        // read the db file and initialize the page from it.
        let file = open_or_create_db_file(db_filepath)?;
        let filesize = get_filesize(db_filepath)?;
        let mut pager = Pager {
            pages: iter::repeat(vec![]).take(max_pages).collect(),
            db_file: file,
            db_filepath: String::from(db_filepath),
            filesize: filesize,
            page_size: page_size,
            max_pages: max_pages,
            num_pages: 0
        };

        pager.num_pages = pager.num_db_pages();

        Ok(pager)
    }

    pub fn get_page(&mut self, page_num: usize) -> Result<&mut Vec<u8>, String> {
        if page_num >= self.max_pages {
            return Err(format!("{} greater than max pages {}.", page_num, self.max_pages));
        }

        if self.pages[page_num].len() == 0 {
            self.pages[page_num] = page::new_leaf_node(true, self.page_size);
            println!("Num db pages: {}", self.num_db_pages());
            if self.num_db_pages() > page_num as u64 {
                // page is present in db file
                self.read_page_from_file(page_num)?;
            } else {
                self.num_pages += 1; // one more page added.
            }
        }

        return Ok(&mut self.pages[page_num])
    }

    pub fn read_page_from_file(&mut self, page_num: usize) -> Result<(), String> {
        // pages are written in order 0,1,2..N
        let page_buffer = &mut self.pages[page_num];
        let page_offset = (page_num * self.page_size) as u64;
        let offset = self.db_file
            .seek(io::SeekFrom::Start(page_offset))
            .map_err(|e| format!("Error in seek to offset {} : error {}", page_offset, e))?;

        if offset != page_offset {
            return Err(format!("Failed to seek to offset {} : offset reached : {}", page_offset, offset))
        }

        let bytes_read = self.db_file
            .read(page_buffer.as_mut())
            .map_err(|e| format!("Error in read to offset {} : error {}", page_offset, e))?;

        if bytes_read != page_buffer.len() {
            format!("Could not read full page_buffer : bytes_read {} : page_buffer_len {}", bytes_read, page_buffer.len());
        }

        Ok(())
    }

    pub fn delete_db_file(&mut self) -> Result<(), String> {
        fs::remove_file(self.db_filepath.as_str()).map_err(|e| format!("Unable to delete db_file : error {}", e.to_string()))
    }

    pub fn flush_page(&mut self, page_pos: usize) -> Result<(), String> {
        if page_pos >= self.pages.len() {
            return Err(format!("Page pos {} is greater than total number of pages.", self.pages.len()))
        }

        let page = &self.pages[page_pos];

        if page.len() != 0 { // never bought in memory and never written ; so flush is no-op
            if page.len() < self.page_size {
                return Err(format!("Unexpected : Page size {} is smaller than the size to flush {}", page.len(), self.page_size))
            }
            self.db_file.seek(io::SeekFrom::Start((page_pos * self.page_size) as u64))
                    .map_err(|e| format!("Failed to seek in db file {}", e))?;

            self.db_file.write(&page[..self.page_size])
                .map_err(|e| format!("Failed to write file {}", e))?;
        }

        Ok(())
    }

    pub fn num_db_pages(&self) -> u64 {
        return self.filesize / (self.page_size as u64);
    }

    pub fn num_pages(&self) -> u64 {
        return self.num_pages;
    }

    pub fn close_db(&mut self) -> Result<(), String> {
        self.db_file.flush()
            .map_err(|e| format!("Failed to flush the db_file to disk : error : {}", e))
    }
}

impl Drop for Pager {
    fn drop(&mut self) {
        self.close_db().expect("Unable to close db.")
    }
}

fn open_or_create_db_file(db_filepath: &str) -> Result<fs::File, String> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(db_filepath);

    match file {
        Err(e) => Err(format!("Unable to open/create db file {} : error {}", db_filepath, e)),
        Ok(file) => Ok(file)
    }
}

fn get_filesize(db_filepath: &str) -> Result<u64, String> {
    Ok(fs::metadata(db_filepath)
        .map_err(|e| format!("Unable to get metadata of file {} : error {}", db_filepath, e))?.len())
}
