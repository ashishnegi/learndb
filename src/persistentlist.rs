use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>
}

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Link<T>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head : None
        }
    }

    pub fn append(&self, val: T) -> List<T> {
        List {
            head : Some(Rc::new(Node{
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
pub struct Iter<'a, T : 'a> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        use std::borrow::Borrow;

        self.next.map(|curr| {
            self.next = curr.next.as_ref().map(|next| &**next);
            &curr.val
        })
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_mut_ops() {
        let upto = 10;
        let mut list2 = List::new();
        for i in 1..upto {
            list2 = list2.append(i);
        }

        assert_eq!(Some(&9), list2.head());

        for i in 1..upto {
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
}