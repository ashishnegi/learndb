mod fileapi;

fn main() {
    println!("Hello, world!");
    assert!(fileapi::create_key("hello", b"world").is_ok());
}
