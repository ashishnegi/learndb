use std::{iter, fs, io};

#[derive(Debug)]
pub struct Pager {
    pages: Vec<Vec<u8>>,
    db_file: fs::File,
    db_filepath: String,
    page_size: usize,
    max_pages: usize
}

impl Pager {
    pub fn new(page_size: usize, max_pages: usize, db_filepath: &str) -> Result<Self, String> {
        // read the db file and initialize the page from it.
        let file = open_or_create_db_file(db_filepath)?;
        Ok(Pager {
            pages: iter::repeat(vec![]).take(max_pages).collect(),
            db_file: file,
            db_filepath: String::from(db_filepath),
            page_size: page_size,
            max_pages: max_pages
        })
    }

    pub fn get_page(&mut self, page_num: usize) -> Result<&mut Vec<u8>, String> {
        if page_num >= self.max_pages {
            return Err(format!("{} greater than max pages {}.", page_num, self.max_pages));
        }

        if self.pages[page_num].len() == 0 {
            self.pages[page_num] = vec![0; self.page_size];
        }

        return Ok(&mut self.pages[page_num])
    }

    pub fn delete_db_file(&mut self) -> Result<(), String> {
        fs::remove_file(self.db_filepath.as_str()).map_err(|e| format!("Unable to delete db_file : error {}", e.to_string()))
    }
}

fn open_or_create_db_file(db_filepath: &str) -> Result<fs::File, String> {
    match fs::File::open(db_filepath) {
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => fs::File::create(db_filepath)
            .map_err(|e| format!("Unable to create db file {} with error {}", db_filepath, e.to_string())),
            _ => Err(format!("Unable to open db file {} : error {}", db_filepath, e.to_string()))
        },
        Ok(file) => Ok(file)
    }
}