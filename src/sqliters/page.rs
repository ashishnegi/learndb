use sqliters::consts;
use std::mem::transmute;

pub fn is_leaf_node(page: &Vec<u8>) -> bool {
    let mut node_type_bytes: [u8; consts::PAGE_TYPE_SIZE] = Default::default();
    node_type_bytes.copy_from_slice(&page[consts::PAGE_TYPE_OFFSET..consts::IS_ROOT_OFFSET]);
    let node_type = unsafe { transmute::<[u8;consts::PAGE_TYPE_SIZE], u8>(node_type_bytes) }.to_be();
    node_type == consts::LEAF_NODE_TYPE
}

fn get_cell_count_ref(page: &Vec<u8>) -> &[u8] {
    &page[consts::NUM_ENTRIES_OFFSET..consts::PAGE_HEADER_SIZE]
}

fn get_cell_count_ref_mut(page: &mut Vec<u8>) -> &mut [u8] {
    &mut page[consts::NUM_ENTRIES_OFFSET..consts::PAGE_HEADER_SIZE]
}

pub fn get_num_cells(page: &Vec<u8>) -> u64 {
    let mut id_bytes: [u8; consts::NUM_ENTRIES_SIZE] = Default::default();
    id_bytes.copy_from_slice(get_cell_count_ref(page));
    unsafe { transmute::<[u8;consts::NUM_ENTRIES_SIZE], u64>(id_bytes) }.to_be()
}

pub fn add_data(page: &mut Vec<u8>, cell_num: u64, data: &Vec<u8>) {
    let cell_offset = consts::PAGE_HEADER_SIZE + (cell_num as usize * consts::CELL_SIZE);
    let cell = &mut page[cell_offset .. cell_offset + consts::CELL_SIZE];

    for pos in 0..consts::KEY_SIZE {
        cell[pos] = data[pos];
    }

    for pos in 0..data.len() {
        cell[pos + consts::VALUE_OFFSET] = data[pos];
    }
}

pub fn increment_cell_count(page: &mut Vec<u8>) {
    let mut count = get_num_cells(page);
    count += 1;

    let count_ref = get_cell_count_ref_mut(page);
    let count_bytes: [u8; consts::NUM_ENTRIES_SIZE] = unsafe { transmute(count.to_be()) };

    for c in 0..consts::NUM_ENTRIES_SIZE {
        count_ref[c] = count_bytes[c];
    }
}

pub fn print_leaf_node(page: &Vec<u8>) {
    if !is_leaf_node(page) {
        return println!("Not leaf node");
    } else {
        print!("leaf_node, ")
    }

    let is_root = is_root_node(page);
    let num_cells = get_num_cells(page);

    print!("root: {}, keys: ", is_root);

    for i in 0 .. num_cells {
        let data = get_cell_data(page, i);
        let mut id_bytes: [u8; consts::ID_SIZE] = Default::default();
        id_bytes.copy_from_slice(&data[0..consts::ID_SIZE]);
        let key = unsafe { transmute::<[u8;4], i32>(id_bytes) }.to_be();
        print!("{}, ", key);
    }
    println!("");
}

pub fn is_root_node(page: &Vec<u8>) -> bool {
    let mut is_root_bytes: [u8; consts::IS_ROOT_SIZE] = Default::default();
    is_root_bytes.copy_from_slice(&page[consts::IS_ROOT_OFFSET..consts::NUM_ENTRIES_OFFSET]);
    let is_root = unsafe { transmute::<[u8;consts::IS_ROOT_SIZE], u8>(is_root_bytes) }.to_be();
    is_root == consts::IS_ROOT_TYPE
}

pub fn get_cell_data(page: &Vec<u8>, cell_num: u64) -> &[u8] {
    let cell_offset = consts::PAGE_HEADER_SIZE + (cell_num as usize * consts::CELL_SIZE);
    &page[cell_offset .. cell_offset + consts::CELL_SIZE]
}

pub fn new_leaf_node(is_root: bool, page_size: usize) -> Vec<u8> {
    let mut bytes = vec![0; page_size];
    let node_type_bytes: [u8; consts::IS_ROOT_OFFSET] = unsafe { transmute(consts::LEAF_NODE_TYPE.to_be()) };
    bytes[consts::PAGE_TYPE_OFFSET .. consts::IS_ROOT_OFFSET].copy_from_slice(&node_type_bytes);

    let is_root_value = if is_root { consts::IS_ROOT_TYPE } else { consts::NON_ROOT_TYPE };
    let is_root_bytes: [u8; consts::IS_ROOT_SIZE] = unsafe { transmute(is_root_value.to_be()) };
    bytes[consts::IS_ROOT_OFFSET .. consts::NUM_ENTRIES_OFFSET].copy_from_slice(&is_root_bytes);

    // rest should be all Zeroes .. num_cells : key : values
    bytes
}