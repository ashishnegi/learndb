use std::mem;

#[derive(Debug)]
pub enum List {
    Nil,
    More(Box<Node>),
}

#[derive(Debug)]
pub struct Node {
    val: i32,
    next: List
}

impl List {
    pub fn new() -> Self {
        List::Nil
    }

    pub fn insert(&mut self, v : i32) {
        let old_head = mem::replace(&mut *self, List::Nil);
        let new_head = List::More(Box::new(Node { val : v, next: old_head}));
        *self = new_head
    }

    pub fn remove(&mut self) -> Option<i32> {
        match mem::replace(&mut *self, List::Nil) {
            List::Nil => {
                None
            },
            List::More(ref mut boxed_node) => {
                let result = Some(boxed_node.val);
                *self = mem::replace(&mut boxed_node.next, List::Nil);
                result
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        while true {
            match self {
                &mut List::Nil => break,
                &mut List::More(ref mut node) => {
                    *self = mem::replace(& mut node.next, List::Nil)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        list.insert(7);
        assert_eq!(Some(7), list.remove());
        assert_eq!(None, list.remove());

        list.insert(1);
        list.insert(2);
        list.insert(3);

        assert_eq!(Some(3), list.remove());
        assert_eq!(Some(2), list.remove());
        assert_eq!(Some(1), list.remove());
        assert_eq!(None, list.remove());
    }

    #[test]
    fn drop_long_list() {
        let mut list = List::new();
        for i in 1..100000 {
            list.insert(i);
        }
    }
}