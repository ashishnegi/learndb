mod fileapi;
mod bplustree;
mod list;

fn main() {
    let key = "hello";
    assert!(fileapi::create_key(key, b"world").is_ok());
    assert!(fileapi::key_exists(key));

    {
        let value = String::from("world").into_bytes();
        assert!(match fileapi::get_value(key) {
            Ok(val) => val == value,
            _ => false
        });
    }

    assert!(fileapi::put_value(key, b"world2").is_ok());
    {
        let value2 = String::from("world2").into_bytes();
        assert!(match fileapi::get_value(key) {
            Ok(val) => val == value2,
            _ => false
        });
    }
    assert!(fileapi::delete_key(key).is_ok());

    println!("bplustree: {:?}", bplustree::Node::new(String::from("hello")));

    let mut list = list::List::new();
    list.insert(7);
    println!("list: {:?}", list);
    list.remove();
    println!("list: {:?}", list);
}
