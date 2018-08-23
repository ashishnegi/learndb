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

pub fn add_data(page: &mut Vec<u8>, cell_pos: u64, data: &Vec<u8>) -> Result<(), String> {
    let num_cells = get_num_cells(page);
    if num_cells < cell_pos {
        return Err(format!("cell_pos {} is higher than number of cells {} already present; You should split before",
            cell_pos, num_cells))
    }

    shift_data(page, cell_pos, num_cells);
    copy_at_cell_pos(page, cell_pos, data);
    Ok(())
}

fn copy_at_cell_pos(page: &mut Vec<u8>, cell_pos: u64, data: &Vec<u8>) {
    let cell_offset = consts::PAGE_HEADER_SIZE + (cell_pos as usize * consts::CELL_SIZE);
    let cell = &mut page[cell_offset .. cell_offset + consts::CELL_SIZE];

    for pos in 0 .. consts::KEY_SIZE {
        cell[pos] = data[pos];
    }

    for pos in 0..data.len() {
        cell[pos + consts::VALUE_OFFSET] = data[pos];
    }
}

fn shift_data(page: &mut Vec<u8>, cell_pos: u64, num_cells: u64) {
    let copy_start_offset = consts::PAGE_HEADER_SIZE + (cell_pos as usize * consts::CELL_SIZE);
    let copy_end_offset = consts::PAGE_HEADER_SIZE + (num_cells as usize * consts::CELL_SIZE);

    let from = Vec::<u8>::from(&page[copy_start_offset .. copy_end_offset]);
    let to = &mut page[copy_start_offset + consts::CELL_SIZE .. copy_end_offset + consts::CELL_SIZE];

    for pos in (0 .. copy_end_offset - copy_start_offset).rev() {
        to[pos] = from[pos];
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
        let mut id_bytes: [u8; consts::KEY_SIZE] = Default::default();
        id_bytes.copy_from_slice(&data[0..consts::KEY_SIZE]);
        // key is i32 ; same as id
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

pub fn find_new_key_pos(page: &Vec<u8>, key: i32) -> Result<u64, String> {
    find_key(page, key).map_err(|e| e.1)
}

pub fn find_key_pos(page: &Vec<u8>, key: i32) -> u64 {
    let pos = find_key(page, key);
    if pos.is_ok() {
        return pos.unwrap()
    } else {
        pos.unwrap_err().0
    }
}

fn find_key(page: &Vec<u8>, key: i32) -> Result<u64, (u64, String)> {
    let num_keys = get_num_cells(page);
    if num_keys == 0 {
        return Ok(0)
    }

    let mut key_start_pos = 0;
    let mut key_end_pos = num_keys - 1;

    while key_start_pos <= key_end_pos {
        let key_pos = key_start_pos + (key_end_pos - key_start_pos) / 2;
        let mid_key = get_key_at(page, key_pos);

        println!("Binary search: key {}, mid_key {}", key, mid_key);

        if key == mid_key {
            return Err((key_pos, format!("Duplicate key {} present at index {} in page", key, key_pos)))
        } else if key > mid_key {
            key_start_pos = key_pos + 1;
        } else if key_pos == 0 {
            break;
        } else {
            key_end_pos = key_pos - 1;
        }
    }

    Ok(key_start_pos)
}

pub fn get_key_at(page: &Vec<u8>, key_pos: u64) -> i32 {
    let mut id_bytes: [u8; consts::KEY_SIZE] = Default::default();
    let key_start_offset = consts::PAGE_HEADER_SIZE + consts::KEY_OFFSET;
    let key_offset = key_start_offset + (key_pos as usize * consts::CELL_SIZE);

    id_bytes.copy_from_slice(&page[key_offset .. key_offset + consts::KEY_SIZE]);
    // key is i32 ; same as id
    unsafe { transmute::<[u8;4], i32>(id_bytes) }.to_be()
}

pub fn deserialize_key(buf: &[u8]) -> i32 {
    let mut id_bytes: [u8; consts::KEY_SIZE] = Default::default();
    id_bytes.copy_from_slice(buf);
    // key is i32 ; same as id
    unsafe { transmute::<[u8;4], i32>(id_bytes) }.to_be()
}