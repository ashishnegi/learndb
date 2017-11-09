use std::ptr;

#[derive(Debug)]
pub struct Queue<T> {
    head: Link<T>,
    tail: *mut Node<T>
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    val: T,
    next: Link<T>
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: ptr::null_mut()
        }
    }

    pub fn push(&mut self, v: T) {
        let mut new_tail = Box::new(Node::new(v));
        let raw_tail: *mut _ = &mut *new_tail;

        if self.tail.is_null() {
            self.head = Some(new_tail);
        } else {
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        }

        self.tail = raw_tail;
    }

    // pub fn pop(&mut self) -> Option<T> {
    //     self.head.take().map(|head| {
    //         let hv = *head;
    //         match hv.next {
    //             None => {
    //                 self.tail = None;
    //                 self.head = None;
    //             },
    //             Some(_) => {
    //                 self.head = hv.next;
    //             }
    //         };

    //         hv.val
    //     })
    // }
}

impl<T> Node<T> {
    pub fn new(v: T) -> Self {
        Node {
            val: v,
            next: None
        }
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     pub fn basics() {
//         let mut q = Queue::new();
//         assert_eq!(None, q.pop());
//         q.push(1);
//         assert_eq!(Some(1), q.pop());
//     }
// }