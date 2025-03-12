// Answer 0

#[test]
fn test_move_index_valid_range_same() {
    let mut index_map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    for i in 0..5 {
        index_map.insert(i, i * 10);
    }
    index_map.move_index(2, 2);
}

#[test]
fn test_move_index_valid_range_up() {
    let mut index_map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    for i in 0..5 {
        index_map.insert(i, i * 10);
    }
    index_map.move_index(2, 4);
}

#[test]
fn test_move_index_valid_range_down() {
    let mut index_map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    for i in 0..5 {
        index_map.insert(i, i * 10);
    }
    index_map.move_index(4, 2);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_from() {
    let mut index_map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    for i in 0..5 {
        index_map.insert(i, i * 10);
    }
    index_map.move_index(5, 2);
}

#[test]
#[should_panic]
fn test_move_index_out_of_bounds_to() {
    let mut index_map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    for i in 0..5 {
        index_map.insert(i, i * 10);
    }
    index_map.move_index(2, 5);
}

#[test]
fn test_move_index_boundary_case_zero() {
    let mut index_map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    index_map.insert(0, 0);
    index_map.insert(1, 10);
    index_map.move_index(0, 0);
}

#[test]
fn test_move_index_boundary_case_max_capacity() {
    let mut index_map: IndexMap<u32, u32, RandomState> = IndexMap::with_capacity(IndexMapCore::<u32, u32>::MAX_ENTRIES_CAPACITY);
    for i in 0..IndexMapCore::<u32, u32>::MAX_ENTRIES_CAPACITY {
        index_map.insert(i, i * 10);
    }
    index_map.move_index(IndexMapCore::<u32, u32>::MAX_ENTRIES_CAPACITY - 1, IndexMapCore::<u32, u32>::MAX_ENTRIES_CAPACITY - 1);
}

#[test]
fn test_move_index_boundary_case_one() {
    let mut index_map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    index_map.insert(0, 0);
    index_map.insert(1, 10);
    index_map.move_index(1, 1);
}

