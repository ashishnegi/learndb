use sqliters::consts;
use std::mem::transmute;

pub fn is_leaf_node(_page: Vec<u8>) -> bool {
    return true;
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
    let id_bytes: [u8; consts::NUM_ENTRIES_SIZE] = unsafe { transmute(count.to_be()) };

    for c in 0..consts::NUM_ENTRIES_SIZE {
        count_ref[c] = id_bytes[c];
    }
}