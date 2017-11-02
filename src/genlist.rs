type Link<T> = Option<Box<Node<T>>>;

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
        List { head : None }
    }

    pub fn insert(&mut self, v: T) {
        let new_head = Some(Box::new(Node {
            val : v,
            next: self.head.take()
        }));

        self.head = new_head;
    }

    pub fn remove(&mut self) -> Option<T> {
        self.head.take().map ( |node| {
            let result = *node;
            self.head = result.next;
            result.val
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.val
        })
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(& self) -> Iter<T> {
        Iter{ next : self.head.as_ref().map(|node| {
            &**node
        })}
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();

        while let Some(mut node) = head {
            head = node.next.take();
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.remove()
    }
}

#[derive(Debug)]
pub struct Iter<'a, T : 'a> {
    next: Option<&'a Node<T>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|boxed_next| {
            self.next = boxed_next.next.as_ref().map(|next| {
                &**next
            });
            &boxed_next.val
        })
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

    #[test]
    fn peeks() {
        let mut list = List::new();
        let upto = 10;
        for i in 1..upto {
            list.insert(i);
        }
        for i in 1..upto {
            assert_eq!(Some(&(upto - i)), list.peek());
            list.remove();
        }
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        let upto = 10;
        for i in 1..upto {
            list.insert(i);
        }

        let mut i = 9;
        let mut iter = list.into_iter();
        while let Some(v) = iter.next() {
            assert_eq!(v, i);
            i = i - 1;
        }

        assert_eq!(None, iter.next());
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        let upto = 10;
        for i in 1..upto {
            list.insert(i);
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