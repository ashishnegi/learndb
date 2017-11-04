use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug)]
pub struct Deque<T> where T: fmt::Debug {
    head: Link<T>,
    tail: Link<T>
}

#[derive(Debug)]
struct Node<T> where T: fmt::Debug {
    val: T,
    next: Link<T>,
    prev: Link<T>
}

impl<T> Node<T> where T: fmt::Debug {
    pub fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val: val,
            next: None,
            prev: None
        }))
    }
}

impl<T> Deque<T> where T: fmt::Debug {
    pub fn new() -> Deque<T> {
        Deque {
            head: None,
            tail: None
        }
    }

    pub fn push_front(&mut self, val: T) {
        let new_head = Node::new(val);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        };
        println!("{:?}", self.head);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            let mut new_head = old_head.borrow_mut().next.take();
            self.head = new_head.clone();
            match new_head {
                None => {
                    self.tail = None;
                },
                Some(ref mut head) => {
                    head.borrow_mut().prev = self.head.clone();
                }
            };

            old_head.borrow_mut().prev = None;
            Rc::try_unwrap(old_head).unwrap().into_inner().val
        })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn push_front() {
        let mut queue : Deque<i32> = Deque::new();
        queue.push_front(1);
        println!("after push : {:?}", queue);
        // queue.push_front(2);

        // assert_eq!(Some(2), queue.pop_front());
        assert_eq!(Some(1), queue.pop_front());
        assert_eq!(None, queue.pop_front());
    }
}