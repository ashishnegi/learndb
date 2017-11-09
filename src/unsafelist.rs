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

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let hv = *head;
            match hv.next {
                None => {
                    self.tail = ptr::null_mut();
                    self.head = None;
                },
                Some(_) => {
                    self.head = hv.next;
                }
            };

            hv.val
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|head|{
            &head.val
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter{ next: self }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_ref().map(|head| {
                &**head
            })
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next : self.head.as_mut().map(|head| &mut **head)
        }
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

#[derive(Debug)]
pub struct IntoIter<T> {
    next: Queue<T>
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.pop()
    }
}

#[derive(Debug)]
pub struct Iter<'a, T : 'a> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|next|{
            self.next = next.next.as_ref().map(|next| {
                &**next
            });
            &next.val
        })
    }
}

#[derive(Debug)]
pub struct IterMut<'a, T : 'a> {
    next: Option<&'a mut Node<T>>
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|next| {
            self.next = next.next.as_mut().map(|next2| {
                &mut **next2
            });
            &mut next.val
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn basics() {
        let mut q = Queue::new();
        assert_eq!(None, q.pop());
        q.push(1);
        assert_eq!(Some(1), q.pop());
        assert_eq!(None, q.pop());
        let mut q2 = q;
        q2.push(2);
        assert_eq!(Some(2), q2.pop());
        assert_eq!(None, q2.pop());
    }

    #[test]
    pub fn into_iter() {
        let mut q = Queue::new();
        q.push(1);
        q.push(2);
        q.push(3);

        {
            let mut iter = q.into_iter();
            assert_eq!(Some(1), iter.next());
            assert_eq!(Some(2), iter.next());
            assert_eq!(Some(3), iter.next());
            assert_eq!(None, iter.next());
        }

        // q.push(1); // compiler error.. :P
    }

    #[test]
    pub fn iter() {
        let mut q = Queue::new();
        q.push(1);
        q.push(2);
        q.push(3);

        {
            let mut iter = q.iter();
            assert_eq!(Some(&1), iter.next());
            assert_eq!(Some(&2), iter.next());
            assert_eq!(Some(&3), iter.next());
            assert_eq!(None, iter.next());
        }

        assert_eq!(Some(&1), q.peek());
        assert_eq!(Some(1), q.pop());
    }

    #[test]
    pub fn iter_mut() {
        let mut q = Queue::new();
        q.push(1);
        q.push(2);
        q.push(3);

        {
            let mut iter = q.iter_mut();
            assert_eq!(Some(&mut 1), iter.next());
            assert_eq!(Some(&mut 2), iter.next());
            assert_eq!(Some(&mut 3), iter.next());
            assert_eq!(None, iter.next());
        }

        assert_eq!(Some(&1), q.peek());
        assert_eq!(Some(1), q.pop());
    }
}