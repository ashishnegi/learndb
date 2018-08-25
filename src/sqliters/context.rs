use sqliters::statement;
use downcast_rs::Downcast;

pub struct Context {
    select_outfn: Box<OutFn>
}

pub trait OutFn: Downcast {
    fn outfn(&mut self, &statement::InsertStatement);
}
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

pub struct AssertSelectOutFn {
    count: i32
}

impl AssertSelectOutFn {
    pub fn new(count: i32) -> Self {
        AssertSelectOutFn{count: count}
    }

    pub fn count(&self) -> i32 {
        self.count
    }
}

impl OutFn for AssertSelectOutFn {
    fn outfn(&mut self, insert: &statement::InsertStatement) {
        assert!(self.count == insert.id(), "self.count {} == insert.id() {}", self.count, insert.id());
        self.count += 1;
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

    pub fn get_out(&self) -> &Box<OutFn> {
        &self.select_outfn
    }
}