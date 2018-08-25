use sqliters::statement;
use downcast_rs::Downcast;

pub struct Context {
    select_outfn: Box<OutFn>
}

pub trait OutFn: Downcast {
    fn outfn(&mut self, &statement::InsertStatement);
}

#[cfg(test)]
impl_downcast!(OutFn);

pub struct ConsoleOutFn {}

impl ConsoleOutFn {
    pub fn new() -> Self {
        ConsoleOutFn{}
    }
}

impl OutFn for ConsoleOutFn {
    fn outfn(&mut self, insert: &statement::InsertStatement) {
        println!("row: {}", insert)
    }
}

impl Context {
    pub fn new(select_outfn: Box<OutFn>) -> Self {
        Context {
            select_outfn: select_outfn
        }
    }

    pub fn select_out(&mut self, insert: &statement::InsertStatement) {
        self.select_outfn.outfn(insert)
    }

    #[cfg(test)]
    pub fn get_out(&self) -> &Box<OutFn> {
        &self.select_outfn
    }
}