use std::mem;

#[derive(Debug)]
pub struct List {
    head: Link
}

#[derive(Debug)]
pub enum Link {
    Nil,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Link
}

impl List {
    pub fn new() -> Self {
        List { head : Link::Nil }
    }

    pub fn insert(&mut self, v : i32) {
        let old_head = mem::replace(&mut self.head, Link::Nil);
        let new_head = Link::More(Box::new(Node { val : v, next: old_head}));
        self.head = new_head
    }

    pub fn remove(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Nil) {
            Link::Nil => {
                None
            },
            Link::More(node) => {
                let result = Some(node.val);
                self.head = node.next;
                result
            }
        }
    }
}