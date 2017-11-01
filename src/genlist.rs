type Link = Option<Box<Node>>;

#[derive(Debug)]
pub struct List {
    head: Link
}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Link
}

impl List {
    pub fn new() -> Self {
        List { head : None }
    }

    pub fn insert(&mut self, v : i32) {
        let new_head = Some(Box::new(Node {
            val : v,
            next: self.head.take()
        }));

        self.head = new_head;
    }

    pub fn remove(&mut self) -> Option<i32> {
        self.head.take().map ( |node| {
            let result = node.val;
            self.head = node.next;
            result
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut head = self.head.take();

        while let Some(mut node) = head {
            head = node.next.take();
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