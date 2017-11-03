use std::sync::Arc;
use std::fmt;

type Link<T> = Option<Arc<Node<T>>>;

#[derive(Debug)]
pub struct List<T> where T: fmt::Debug {
    head: Link<T>
}

#[derive(Debug)]
struct Node<T> where T: fmt::Debug {
    val: T,
    next: Link<T>
}

impl<T> List<T> where T: fmt::Debug {
    pub fn new() -> Self {
        List {
            head : None
        }
    }

    pub fn append(&self, val: T) -> List<T> {
        List {
            head : Some(Arc::new(Node{
                val : val,
                next : self.head.clone()
            }))
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head : self.head.as_ref().and_then(|head| {
                head.next.clone()
            })
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|head| {
            &head.val
        })
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            next : self.head.as_ref().map(|head| {
                &**head
            })
        }
    }
}

#[derive(Debug)]
pub struct Iter<'a, T : 'a> where T: fmt::Debug {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> where T: fmt::Debug {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|curr| {
            self.next = curr.next.as_ref().map(|next| &**next);
            &curr.val
        })
    }
}

impl<T> Drop for List<T> where T: fmt::Debug {
    fn drop(&mut self) {
        let mut curr_list = self.head.take();
        while let Some(node) = curr_list {
            match Arc::try_unwrap(node) {
                Err(_) => {
                    break
                },
                Ok(mut x) => {
                    curr_list = x.next.take()
                }
            }
        }
    }
}

impl<T> Drop for Node<T> where T: fmt::Debug {
    fn drop(&mut self) {
        println!("Node drop : {:?}", self.val);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        List::new().append(1);
    }
    #[test]
    fn basic_mut_ops() {
        let upto = 10;
        let mut list2 = List::new();
        for i in 1..upto {
            list2 = list2.append(i);
        }

        assert_eq!(Some(&9), list2.head());

        for _ in 1..upto {
            list2 = list2.tail();
        }

        assert_eq!(None, list2.head());
    }

    #[test]
    fn chaining() {
        let list = List::new().append(1).append(2).append(3);
        assert_eq!(Some(&3), list.head());
        let list = list.tail();
        assert_eq!(Some(&2), list.head());
        let list = list.tail().tail();
        assert_eq!(None, list.head());
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        let upto = 10;
        for i in 1..upto {
            list = list.append(i);
        }

        let mut i = 9;
        let mut iter = list.iter();
        while let Some(v) = iter.next() {
            assert_eq!(v, &i);
            i = i - 1;
        }

        assert_eq!(None, iter.next());
    }

    #[test]
    fn drop() {
        let mut list = List::new();
        let upto = 100000;
        for i in 1..upto {
            list = list.append(i);
        }
    }
}