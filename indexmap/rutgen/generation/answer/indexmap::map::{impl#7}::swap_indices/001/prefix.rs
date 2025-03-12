// Answer 0

#[test]
fn test_swap_indices_valid_range() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.insert(3, 30);
    map.swap_indices(0, 1);
}

#[test]
fn test_swap_indices_equal() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.swap_indices(0, 0);
}

#[test]
fn test_swap_indices_first_and_last() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.swap_indices(0, 1);
}

#[test]
fn test_swap_indices_last_and_first() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.swap_indices(1, 0);
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds_high() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.swap_indices(0, 2);
}

#[test]
#[should_panic]
fn test_swap_indices_out_of_bounds_low() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    map.swap_indices(2, 1);
}

#[test]
#[should_panic]
fn test_swap_indices_empty_map() {
    let mut map: IndexMap<u32, u32, RandomState> = IndexMap::new();
    map.swap_indices(0, 0);
}

