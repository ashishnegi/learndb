use std::mem;

#[derive(Debug)]
pub struct Queue<'a, T : 'a> {
    head: Link<T>,
    tail: Option<&'a mut Node<T>>
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    val: T,
    next: Link<T>
}

impl<'a, T> Queue<'a, T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: None
        }
    }

    pub fn push(&'a mut self, v: T) {
        let mut new_tail = Box::new(Node::new(v));
        let new_tail_ref = match self.tail.take() {
            None => {
                self.head = Some(new_tail);
                self.head.as_mut().map(|head| &mut **head)
            },
            Some(mut tail) => {
                tail.next = Some(new_tail);
                tail.next.as_mut().map(|next| &mut **next)
            }
        };

        self.tail = new_tail_ref;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let hv = *head;
            match hv.next {
                None => {
                    self.tail = None;
                    self.head = None;
                },
                Some(_) => {
                    self.head = hv.next;
                }
            };

            hv.val
        })
    }
}

impl<T> Node<T> {
    pub fn new(v: T) -> Self {
        Node {
            val: v,
            next: None
        }
    }
}