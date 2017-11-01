use std::mem;

#[derive(Debug)]
pub enum List {
    Nil,
    More(Node),
}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Box<List>
}

impl List {
    pub fn new() -> Self {
        List::Nil
    }

    pub fn insert(&mut self, v : i32) {
        let old_self = mem::replace(self, List::Nil);
        let new_list = List::More(Node { val : v, next: Box::new(old_self)});
        self = &mut new_list
    }
}