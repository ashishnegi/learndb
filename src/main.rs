mod fileapi;
mod bplustree;
mod list;
mod genlist;
mod consistency;
mod persistentlist;
mod deque;
mod unsafelist;

fn main() {
    let key = "hello";
    use fileapi::Storage;

    let storage = fileapi::FileStorage::new(String::from("./"));
    assert!(storage.put_value(key, b"world").is_ok());
    assert!(storage.key_exists(key));

    {
        let value = String::from("world").into_bytes();
        assert!(match storage.get_value(key) {
            Ok(val) => val == value,
            _ => false
        });
    }

    assert!(storage.put_value(key, b"world2").is_ok());
    {
        let value2 = String::from("world2").into_bytes();
        assert!(match storage.get_value(key) {
            Ok(val) => val == value2,
            _ => false
        });
    }
    assert!(storage.delete_key(key).is_ok());

    println!("bplustree: {:?}", bplustree::Node::new(String::from("hello")));

    {
        let mut list = list::List::new();
        list.insert(7);
        println!("list: {:?}", list);
        list.remove();
        println!("list: {:?}", list);
    }

    {
        let mut list = genlist::List::new();
        list.insert(7);
        println!("list: {:?}", list);
        list.remove();
        println!("list: {:?}", list);
    }

    {
        consistency::Consistency::new(&storage);
    }

    {
        let list = persistentlist::List::new().append(1);
        println!("persistentlist: {:?}", list);
        list.tail();
        println!("persistentlist: {:?}", list);
        let list2 = list.tail();
        println!("persistentlist: {:?}", list2);
    }

    {
        let mut queue = deque::Deque::new();
        queue.push_front(1);
        queue.push_front(2);
        assert_eq!(2, *queue.peek_front().unwrap());
        assert_eq!(Some(2), queue.pop_front());
        assert_eq!(1, *queue.peek_front().unwrap());
    }

    {
        let mut queue = unsafelist::Queue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(1, *queue.peek().unwrap());
        assert_eq!(Some(1), queue.pop());
        assert_eq!(2, *queue.peek().unwrap());
    }
}
