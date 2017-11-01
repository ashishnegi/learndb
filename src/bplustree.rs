use std::rc::Rc;
use std::ptr;

const BRANCH_FACTOR: usize = 4;

// #[derive(Debug)]
// struct SizedVec<T, Size> {
//     data: Vec<T>
// }

// impl SizedVec<T, Size> {
//     fn create() {
//         SizedVec<T,Size> { Vec<T>(Size) }
//     }
// }

#[derive(Debug)]
struct InternalNode {
    keys: Vec<String>, // convert to SizedVec
    children: Vec<Rc<Node>>
}

#[derive(Debug)]
enum LeafNode {
    Nil,
    LeafData(LeafData)
}

#[derive(Debug)]
struct LeafData {
    key: Vec<String>,
    data: Vec<Data>,
    next: Rc<LeafNode>,
}

#[derive(Debug)]
struct Data {
    key: String
}

#[derive(Debug)]
pub enum Node {
    InternalNode(InternalNode),
    LeafNode(LeafNode),
}

impl Node {
    pub fn new(key: String) -> Self {
        let leaf_data = LeafData {
            key: vec![key.clone()],
            data: vec![Data { key }],
            next: Rc::new(LeafNode::Nil)
        };

        Node::LeafNode(LeafNode::LeafData(leaf_data))
    }

    pub fn insert(&mut self, key: String, value: &[u8])
    {
        match self {
            &mut Node::LeafNode(ref leaf_node) => {

            }
            _ => {}
        }
    }
}
