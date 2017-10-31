use std::rc::Rc;
use std::ptr;

const BRANCH_FACTOR: usize = 1;

#[derive(Debug)]
struct InternalNode {
    keys: [String; BRANCH_FACTOR],
    children: [Rc<Node>; BRANCH_FACTOR + 1],
}

#[derive(Debug)]
enum LeafNode {
    Nil,
    LeafData(LeafData)
}

#[derive(Debug)]
struct LeafData {
    key: [String; BRANCH_FACTOR],
    data: [Data; BRANCH_FACTOR],
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

pub fn create_root(key: String) -> Node {
    let leaf_data = LeafData {
        key: [key.clone()],
        data: [Data { key }],
        next: Rc::new(LeafNode::Nil)
    };

    Node::LeafNode(LeafNode::LeafData(leaf_data))
}