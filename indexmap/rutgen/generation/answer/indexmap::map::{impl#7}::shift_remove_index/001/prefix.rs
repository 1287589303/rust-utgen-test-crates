// Answer 0

#[test]
fn test_shift_remove_index_valid() {
    let mut index_map: IndexMap<u32, String, RandomState> = IndexMap::with_capacity(10);
    index_map.insert(1, "One".to_string());
    index_map.insert(2, "Two".to_string());
    index_map.insert(3, "Three".to_string());
    let result = index_map.shift_remove_index(1);
}

#[test]
fn test_shift_remove_index_boundary_low() {
    let mut index_map: IndexMap<u32, String, RandomState> = IndexMap::with_capacity(10);
    index_map.insert(1, "One".to_string());
    let result = index_map.shift_remove_index(0);
}

#[test]
fn test_shift_remove_index_boundary_high() {
    let mut index_map: IndexMap<u32, String, RandomState> = IndexMap::with_capacity(10);
    index_map.insert(1, "One".to_string());
    index_map.insert(2, "Two".to_string());
    let result = index_map.shift_remove_index(1);
}

#[test]
#[should_panic]
fn test_shift_remove_index_out_of_bounds_high() {
    let mut index_map: IndexMap<u32, String, RandomState> = IndexMap::with_capacity(10);
    index_map.insert(1, "One".to_string());
    index_map.insert(2, "Two".to_string());
    let _ = index_map.shift_remove_index(2);
}

#[test]
#[should_panic]
fn test_shift_remove_index_out_of_bounds_negative() {
    let mut index_map: IndexMap<u32, String, RandomState> = IndexMap::with_capacity(10);
    let _ = index_map.shift_remove_index(0);
}

