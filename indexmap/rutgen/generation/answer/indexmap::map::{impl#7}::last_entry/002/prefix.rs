// Answer 0

#[test]
fn test_last_entry_with_one_element() {
    let mut map: IndexMap<i32, i32> = IndexMap::new();
    map.insert(1, 10);
    let entry = map.last_entry();
}

#[test]
fn test_last_entry_with_multiple_elements() {
    let mut map: IndexMap<i32, i32> = IndexMap::new();
    for i in 0..10 {
        map.insert(i, i * 10);
    }
    let entry = map.last_entry();
}

#[test]
fn test_last_entry_with_boundary_capacity() {
    let mut map: IndexMap<u32, u32> = IndexMap::new();
    for i in 0..u32::MAX {
        map.insert(i, i * 2);
        if map.len() == 10 { // Using a small number for practicality
            break;
        }
    }
    let entry = map.last_entry();
}

