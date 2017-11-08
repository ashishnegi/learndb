use std::rc::Rc;
use std::cell::Ref;
use std::cell::RefCell;
use std::fmt;
use std::iter::DoubleEndedIterator;

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
        }
    }

    pub fn push_back(&mut self, val: T) {
        let new_tail = Node::new(val);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
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
                    head.borrow_mut().prev = None;
                }
            };

            old_head.borrow_mut().prev.take();
            Rc::try_unwrap(old_head).unwrap().into_inner().val
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            let mut new_tail = old_tail.borrow_mut().prev.take();
            self.tail = new_tail.clone();
            match new_tail {
                None => {
                    self.head = None;
                },
                Some(ref mut tail) => {
                    tail.borrow_mut().next = None;
                }
            };

            old_tail.borrow_mut().next.take();
            Rc::try_unwrap(old_tail).unwrap().into_inner().val
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head.as_ref().map(|head| {
            Ref::map(head.borrow(), |node| {
                &node.val
            })
        })
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail.as_ref().map(|tail| {
            Ref::map(tail.borrow(), |node| {
                &node.val
            })
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            queue: self
        }
    }
}

#[derive(Debug)]
pub struct IntoIter<T> where T: fmt::Debug {
    queue: Deque<T>
}

impl<T> Iterator for IntoIter<T> where T: fmt::Debug {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> where T: fmt::Debug {
    fn next_back(&mut self) -> Option<T> {
        self.queue.pop_back()
    }
}

// #[derive(Debug)]
// struct Iter<'a, T: 'a> where T: fmt::Debug {
//     curr: Option<RefCell<Node<T>>>
// }

// impl<'a, T> Iterator for Iter<'a, T> where T: fmt::Debug {
//     type Item = Ref<'a, T>;
//     fn next(&mut self) -> Option<Self::Item> {
//         use std::ops::Deref;
//         self.curr.take().map(|curr| {
//             let curr = curr.borrow();
//             self.curr = curr.next.as_ref().map(|next| &**next);
//             Ref::map(curr, |node| &node.val)
//         })
//     }
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn basics_front() {
        let mut queue : Deque<i32> = Deque::new();
        queue.push_front(1);
        queue.push_front(2);

        assert_eq!(2, *queue.peek_front().unwrap());
        assert_eq!(Some(2), queue.pop_front());

        assert_eq!(1, *queue.peek_front().unwrap());
        assert_eq!(Some(1), queue.pop_front());

        assert_eq!(None, queue.pop_front());
    }

    #[test]
    pub fn basics_back() {
        let mut queue : Deque<i32> = Deque::new();
        queue.push_back(1);
        queue.push_back(2);

        assert_eq!(2, *queue.peek_back().unwrap());
        assert_eq!(Some(2), queue.pop_back());

        assert_eq!(1, *queue.peek_back().unwrap());
        assert_eq!(Some(1), queue.pop_back());

        assert_eq!(None, queue.pop_back());
    }

    #[test]
    pub fn empty_list_front() {
        let mut queue : Deque<i32> = Deque::new();
        assert_eq!(None, queue.pop_front());
        assert_eq!(None, queue.pop_front());
    }

    #[test]
    pub fn empty_list_back() {
        let mut queue : Deque<i32> = Deque::new();
        assert_eq!(None, queue.pop_back());
        assert_eq!(None, queue.pop_back());
    }

    #[test]
    pub fn into_iter() {
        let mut queue = Deque::new();
        queue.push_front(1);
        queue.push_back(2);

        let mut iter = queue.into_iter();
        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(2), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    pub fn double_ended_iter() {
        let mut queue = Deque::new();
        queue.push_front(1);
        queue.push_back(2);
        queue.push_back(3);

        let mut iter = queue.into_iter();
        assert_eq!(Some(1), iter.next());
        assert_eq!(Some(3), iter.next_back());
        assert_eq!(Some(2), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next_back());
    }
}