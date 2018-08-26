use sqliters::consts;
use std::mem::transmute;

#[derive(Debug, Clone)]
pub enum NodeType {
    Leaf,
    Internal
}

#[derive(Debug, Clone)]
pub struct Page {
    is_root: bool,
    node_type: NodeType,
    data: Vec<u8>,
    num_cells: u64
}

impl Page {
    pub fn new(data: Vec<u8>) -> Self {
        let num_cells = get_num_cells(&data);
        Page {
            is_root: is_root_node(&data),
            node_type: if is_leaf_node(&data) { NodeType::Leaf } else { NodeType::Internal },
            data: data,
            num_cells: num_cells
        }
    }

    pub fn empty() -> Self {
        Page {
            is_root: false,
            node_type: NodeType::Leaf,
            data: vec![],
            num_cells: 0
        }
    }

    pub fn new_leaf(is_root: bool, page_size: usize) -> Self {
        Page {
            is_root: is_root,
            node_type: NodeType::Leaf,
            data: new_leaf_node(is_root, page_size),
            num_cells: 0
        }
    }

    pub fn new_root(page_size: usize, left_page_num: u64, right_page_num: u64, left: &Page) -> Self {
        let max_key = left.max_key();
        let mut bytes = vec![0; page_size];

        let header_offset = consts::PAGE_HEADER_SIZE;

        let right_start_offset = consts::INTERNAL_NODE_RIGHT_PAGE_NUM_OFFSET;
        let right_end_offset = right_start_offset + consts::INTERNAL_NODE_PAGE_NUM_SIZE;
        let right_page_bytes: [u8; consts::INTERNAL_NODE_PAGE_NUM_SIZE] = unsafe { transmute(right_page_num.to_be()) };
        bytes[right_start_offset .. right_end_offset]
            .copy_from_slice(&right_page_bytes);

        let left_page_bytes: [u8; consts::INTERNAL_NODE_PAGE_NUM_SIZE] = unsafe { transmute(left_page_num.to_be()) };
        let left_page_num_start_offset = consts::INTERNAL_NODE_CELL_START_OFFSET + consts::INTERNAL_NODE_LEFT_PAGE_NUM_OFFSET;
        let left_page_num_end_offset = consts::INTERNAL_NODE_CELL_START_OFFSET + consts::INTERNAL_NODE_KEY_OFFSET;
        bytes[left_page_num_start_offset .. left_page_num_end_offset]
            .copy_from_slice(&left_page_bytes);

        let key_start_offset = consts::INTERNAL_NODE_CELL_START_OFFSET + consts::INTERNAL_NODE_KEY_OFFSET;
        let key_end_offset = key_start_offset + consts::INTERNAL_NODE_KEY_SIZE;
        let key_bytes: [u8; consts::KEY_SIZE] = unsafe {transmute(max_key.to_be())};
        bytes[key_start_offset .. key_end_offset]
            .copy_from_slice(&key_bytes);

        Page {
            is_root: true,
            node_type: NodeType::Internal,
            data: bytes,
            num_cells: 1
        }
    }

    pub fn set_non_root(&mut self) {
        self.is_root = false;
    }

    pub fn max_key(&self) -> i32 {
        self.get_key_at(self.num_cells() - 1)
    }

    pub fn is_root(&self) -> bool {
        self.is_root
    }

    pub fn is_leaf(&self) -> bool {
        match self.node_type {
            NodeType::Leaf => true,
            _ => false
        }
    }

    pub fn num_cells(&self) -> u64 {
        self.num_cells
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn get_data(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    pub fn get_key_at(&self, key_pos: u64) -> i32 {
        match self.node_type {
            NodeType::Leaf => leaf_get_key_at(&self.data, key_pos),
            NodeType::Internal => internal_node_get_key_at(&self.data, key_pos)
        }
    }

    pub fn increment_cell_count(&mut self) {
        self.num_cells += 1
    }

    pub fn add_data(&mut self, cell_pos: u64, data: &Vec<u8>) -> Result<(), String> {
        if self.num_cells < cell_pos {
            return Err(format!("cell_pos {} is higher than number of cells {} already present; You should split before : page: {}",
                cell_pos, self.num_cells, self.print()))
        }

        match self.node_type {
            NodeType::Leaf => {
                leaf_shift_data(&mut self.data, cell_pos, self.num_cells);
                leaf_copy_at_cell_pos(&mut self.data, cell_pos, data);
                Ok(())
            },
            NodeType::Internal => panic!("add_data not implemented for internal node")
        }
    }

    pub fn print(&self) -> bool {
        print!("leaf: {}, root: {}, num_cells: {}, keys: ", self.is_leaf(), self.is_root, self.num_cells);

        for i in 0 .. self.num_cells {
            print!("{}, ", self.get_key_at(i));
        }

        println!("");
        true
    }

    pub fn page_size(&self) -> usize {
        self.data.len()
    }

    pub fn flush(&mut self) {
        // set header
        set_is_root(&mut self.data, self.is_root);
        set_node_type(&mut self.data, &self.node_type);
        set_cell_count(&mut self.data, self.num_cells as usize);
    }

    pub fn split(&mut self) -> Page {
        match self.node_type {
            NodeType::Leaf => {
                let new_my_num_cells = self.num_cells / 2;
                let mut new_page = Page {
                    is_root: false,
                    node_type: NodeType::Leaf,
                    data: new_leaf_node(false, self.page_size()),
                    num_cells: self.num_cells - new_my_num_cells
                };

                let to_move_byte_size = new_page.num_cells as usize * consts::CELL_SIZE;
                let my_start_offset = consts::PAGE_HEADER_SIZE + (new_my_num_cells as usize * consts::CELL_SIZE);
                let my_end_offset = my_start_offset + to_move_byte_size;

                new_page.data[consts::PAGE_HEADER_SIZE .. consts::PAGE_HEADER_SIZE + to_move_byte_size]
                    .copy_from_slice(&self.data[my_start_offset .. my_end_offset ]);

                self.num_cells = new_my_num_cells;
                new_page
            },
            NodeType::Internal => {
                panic!("split is not implemented for internal nodes")
            }
        }
    }

    pub fn find_key_pos(&self, key: i32) -> u64 {
        match self.node_type {
            NodeType::Leaf => self.leaf_find_key(key),
            NodeType::Internal => self.internal_node_find_key(key)
        }
    }

    fn internal_node_find_key(&self, key: i32) -> u64 {
        panic!("internal_node_find_key not implemented")
    }

    fn leaf_find_key(&self, key: i32) -> u64 {
        let num_keys = self.num_cells();

        if num_keys == 0 {
            return 0;
        }

        let mut key_start_pos = 0;
        let mut key_end_pos = num_keys - 1;

        while key_start_pos <= key_end_pos {
            let key_pos = key_start_pos + (key_end_pos - key_start_pos) / 2;
            let mid_key = self.get_key_at(key_pos);

            println!("Binary search: key {}, mid_key {}, pos {}, num_keys {}", key, mid_key, key_pos, num_keys);

            if key == mid_key {
                return key_pos;
            } else if key > mid_key {
                key_start_pos = key_pos + 1;
            } else if key_pos == 0 {
                break;
            } else {
                key_end_pos = key_pos - 1;
            }
        }

        return key_start_pos;
    }
}

fn is_leaf_node(page: &Vec<u8>) -> bool {
    let mut node_type_bytes: [u8; consts::PAGE_TYPE_SIZE] = Default::default();
    node_type_bytes.copy_from_slice(&page[consts::PAGE_TYPE_OFFSET..consts::IS_ROOT_OFFSET]);
    let node_type = unsafe { transmute::<[u8;consts::PAGE_TYPE_SIZE], u8>(node_type_bytes) }.to_be();
    node_type == consts::LEAF_NODE_TYPE
}

fn set_node_type(page: &mut Vec<u8>, node_type: &NodeType) {
    let node_type_value = match node_type {
        NodeType::Leaf => consts::LEAF_NODE_TYPE,
        NodeType::Internal => consts::NONLEAF_NODE_TYPE
    };
    let node_type_bytes: [u8; consts::PAGE_TYPE_SIZE] = unsafe { transmute(node_type_value.to_be()) };
    page[consts::PAGE_TYPE_OFFSET..consts::IS_ROOT_OFFSET]
        .copy_from_slice(&node_type_bytes);
}

fn get_cell_count_ref(page: &Vec<u8>) -> &[u8] {
    &page[consts::NUM_ENTRIES_OFFSET..consts::PAGE_HEADER_SIZE]
}

fn get_cell_count_ref_mut(page: &mut Vec<u8>) -> &mut [u8] {
    &mut page[consts::NUM_ENTRIES_OFFSET..consts::PAGE_HEADER_SIZE]
}

fn get_num_cells(page: &Vec<u8>) -> u64 {
    let mut id_bytes: [u8; consts::NUM_ENTRIES_SIZE] = Default::default();
    id_bytes.copy_from_slice(get_cell_count_ref(page));
    unsafe { transmute::<[u8;consts::NUM_ENTRIES_SIZE], u64>(id_bytes) }.to_be()
}

fn leaf_copy_at_cell_pos(page: &mut Vec<u8>, cell_pos: u64, data: &Vec<u8>) {
    let cell_offset = consts::PAGE_HEADER_SIZE + (cell_pos as usize * consts::CELL_SIZE);
    let cell = &mut page[cell_offset .. cell_offset + consts::CELL_SIZE];

    for pos in 0 .. consts::KEY_SIZE {
        cell[pos] = data[pos];
    }

    for pos in 0..data.len() {
        cell[pos + consts::VALUE_OFFSET] = data[pos];
    }
}

fn shift_data(page: &mut Vec<u8>, copy_start_offset: usize, copy_end_offset: usize) {
    let from = Vec::<u8>::from(&page[copy_start_offset .. copy_end_offset]);
    let to = &mut page[copy_start_offset + consts::CELL_SIZE .. copy_end_offset + consts::CELL_SIZE];

    for pos in (0 .. copy_end_offset - copy_start_offset).rev() {
        to[pos] = from[pos];
    }
}

fn leaf_shift_data(page: &mut Vec<u8>, cell_pos: u64, num_cells: u64) {
    let copy_start_offset = consts::PAGE_HEADER_SIZE + (cell_pos as usize * consts::CELL_SIZE);
    let copy_end_offset = consts::PAGE_HEADER_SIZE + (num_cells as usize * consts::CELL_SIZE);
    shift_data(page, copy_start_offset, copy_end_offset)
}

fn internal_node_shift_data(page: &mut Vec<u8>, cell_pos: u64, num_cells: u64) {
    panic!("internal_node_shift_data not implemented")
    // let copy_start_offset = consts::INTERNAL_NODE_CELL_START_OFFSET + (cell_pos as usize * consts::INTERNAL_NODE_CELL_SIZE);
    // let copy_end_offset = consts::INTERNAL_NODE_CELL_START_OFFSET + (num_cells as usize * consts::INTERNAL_NODE_CELL_SIZE);
    // shift_data(page, copy_start_offset, copy_end_offset)
}

fn set_cell_count(page: &mut Vec<u8>, count: usize) {
    let count_ref = get_cell_count_ref_mut(page);
    let count_bytes: [u8; consts::NUM_ENTRIES_SIZE] = unsafe { transmute(count.to_be()) };

    for c in 0..consts::NUM_ENTRIES_SIZE {
        count_ref[c] = count_bytes[c];
    }
}

fn is_root_node(page: &Vec<u8>) -> bool {
    let mut is_root_bytes: [u8; consts::IS_ROOT_SIZE] = Default::default();
    is_root_bytes.copy_from_slice(&page[consts::IS_ROOT_OFFSET..consts::NUM_ENTRIES_OFFSET]);
    let is_root = unsafe { transmute::<[u8;consts::IS_ROOT_SIZE], u8>(is_root_bytes) }.to_be();
    is_root == consts::IS_ROOT_TYPE
}

fn set_is_root(page: &mut Vec<u8>, is_root: bool) {
    let is_root_val = if is_root { consts::IS_ROOT_TYPE } else { consts::NON_ROOT_TYPE };
    let is_root_bytes: [u8; consts::IS_ROOT_SIZE] = unsafe {transmute(is_root_val.to_be())};
    page[consts::IS_ROOT_OFFSET..consts::NUM_ENTRIES_OFFSET]
        .copy_from_slice(&is_root_bytes);
}

fn new_leaf_node(is_root: bool, page_size: usize) -> Vec<u8> {
    let mut bytes = vec![0; page_size];
    let node_type_bytes: [u8; consts::IS_ROOT_OFFSET] = unsafe { transmute(consts::LEAF_NODE_TYPE.to_be()) };
    bytes[consts::PAGE_TYPE_OFFSET .. consts::IS_ROOT_OFFSET].copy_from_slice(&node_type_bytes);

    let is_root_value = if is_root { consts::IS_ROOT_TYPE } else { consts::NON_ROOT_TYPE };
    let is_root_bytes: [u8; consts::IS_ROOT_SIZE] = unsafe { transmute(is_root_value.to_be()) };
    bytes[consts::IS_ROOT_OFFSET .. consts::NUM_ENTRIES_OFFSET].copy_from_slice(&is_root_bytes);

    // rest should be all Zeroes .. num_cells : key : values
    bytes
}

fn get_key_at(page: &Vec<u8>, key_start_offset: usize, cell_size: usize, key_pos: u64) -> i32 {
    let mut id_bytes: [u8; consts::KEY_SIZE] = Default::default();
    let key_offset = key_start_offset + (key_pos as usize * cell_size);

    id_bytes.copy_from_slice(&page[key_offset .. key_offset + consts::KEY_SIZE]);
    // key is i32 ; same as id
    unsafe { transmute::<[u8;4], i32>(id_bytes) }.to_be()
}

fn leaf_get_key_at(page: &Vec<u8>, key_pos: u64) -> i32 {
    let key_start_offset = consts::PAGE_HEADER_SIZE + consts::KEY_OFFSET;
    get_key_at(page, key_start_offset, consts::CELL_SIZE, key_pos)
}

fn internal_node_get_key_at(page: &Vec<u8>, key_pos: u64) -> i32 {
    let key_start_offset = consts::INTERNAL_NODE_CELL_START_OFFSET + consts::INTERNAL_NODE_KEY_OFFSET;
    get_key_at(page, key_start_offset, consts::INTERNAL_NODE_CELL_SIZE, key_pos)
}

pub fn deserialize_key(buf: &[u8]) -> i32 {
    let mut id_bytes: [u8; consts::KEY_SIZE] = Default::default();
    id_bytes.copy_from_slice(buf);
    // key is i32 ; same as id
    unsafe { transmute::<[u8;4], i32>(id_bytes) }.to_be()
}