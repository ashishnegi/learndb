use std::mem;

// Struct related
pub const ID_SIZE: usize = mem::size_of::<i32>();
pub const USERNAME_SIZE: usize = 32;
pub const EMAIL_SIZE: usize = 32;

pub const ID_OFFSET: usize = 0;
pub const USERNAME_OFFSET: usize = ID_OFFSET + ID_SIZE;
pub const EMAIL_OFFSET: usize = USERNAME_OFFSET + USERNAME_SIZE;
pub const INSERT_STATEMENT_SIZE: usize = EMAIL_OFFSET + EMAIL_SIZE;

// Page
pub const PAGE_SIZE: usize = 2046;
pub const TABLE_MAX_PAGES: usize = 3;
pub const ROW_SIZE: usize = INSERT_STATEMENT_SIZE;

// Header size
pub const PAGE_TYPE_SIZE: usize = mem::size_of::<u8>();
pub const PAGE_TYPE_OFFSET: usize = 0;
pub const IS_ROOT_SIZE: usize = mem::size_of::<u8>();
pub const IS_ROOT_OFFSET: usize = PAGE_TYPE_OFFSET + PAGE_TYPE_SIZE;
pub const NUM_ENTRIES_SIZE: usize = mem::size_of::<u64>();
pub const NUM_ENTRIES_OFFSET: usize = IS_ROOT_OFFSET + IS_ROOT_SIZE;
pub const PAGE_HEADER_SIZE: usize = NUM_ENTRIES_OFFSET + NUM_ENTRIES_SIZE;

// Leaf node : Offsets in body
pub const KEY_SIZE: usize = ID_SIZE;
pub const KEY_OFFSET: usize = 0;
pub const VALUE_SIZE: usize = ROW_SIZE;
pub const VALUE_OFFSET: usize = KEY_OFFSET + KEY_SIZE;
pub const CELL_SIZE: usize = VALUE_OFFSET + VALUE_SIZE;
pub const CELLS_PER_PAGE: usize = (PAGE_SIZE - PAGE_HEADER_SIZE) / CELL_SIZE;

pub const TABLE_MAX_ROWS: usize = CELLS_PER_PAGE * TABLE_MAX_PAGES;

pub const LEAF_NODE_TYPE: u8 = 1;
pub const NONLEAF_NODE_TYPE: u8 = LEAF_NODE_TYPE + 1;
pub const IS_ROOT_TYPE: u8 = 67;
pub const NON_ROOT_TYPE: u8 = IS_ROOT_TYPE - 1;

// Internal node
pub const INTERNAL_NODE_PAGE_NUM_SIZE: usize = mem::size_of::<u64>();
pub const INTERNAL_NODE_KEY_SIZE: usize = ID_SIZE;
pub const INTERNAL_NODE_LEFT_PAGE_NUM_OFFSET: usize = 0;
pub const INTERNAL_NODE_KEY_OFFSET: usize = INTERNAL_NODE_LEFT_PAGE_NUM_OFFSET + INTERNAL_NODE_PAGE_NUM_SIZE;
pub const INTERNAL_NODE_L_K_R_SIZE: usize = 2 * INTERNAL_NODE_PAGE_NUM_SIZE + INTERNAL_NODE_KEY_SIZE;

// HEADER : RIGHT-MOST-PAGE-NUM : [ CELL [ LEFT : KEY ] ] : [ CELL [ LEFT : KEY ] ]
pub const INTERNAL_NODE_RIGHT_PAGE_NUM_OFFSET: usize = PAGE_HEADER_SIZE;
pub const INTERNAL_NODE_CELL_SIZE: usize = INTERNAL_NODE_PAGE_NUM_SIZE + INTERNAL_NODE_KEY_SIZE;
pub const INTERNAL_NODE_CELL_START_OFFSET: usize = PAGE_HEADER_SIZE + INTERNAL_NODE_PAGE_NUM_SIZE;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consts() {
        assert!(TABLE_MAX_ROWS == 84, "TABLE_MAX_ROWS {}", TABLE_MAX_ROWS);
        assert!(CELLS_PER_PAGE == 28, "CELLS_PER_PAGE {}", CELLS_PER_PAGE);
    }
}
