// Answer 0

#[test]
fn test_swap_remove_index_empty() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let result = index_map.swap_remove_index(0);
}

#[test]
fn test_swap_remove_index_single_element() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.push_entry(0, 1, 10);
    let result = index_map.swap_remove_index(0);
}

#[test]
fn test_swap_remove_index_multiple_elements() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.push_entry(0, 1, 10);
    index_map.push_entry(1, 2, 20);
    index_map.push_entry(2, 3, 30);
    let result1 = index_map.swap_remove_index(1);
    let result2 = index_map.swap_remove_index(0);
    let result3 = index_map.swap_remove_index(1); // after previous removals, this index is now 1.
}

#[test]
#[should_panic]
fn test_swap_remove_index_out_of_bounds() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.push_entry(0, 1, 10);
    let result = index_map.swap_remove_index(2);
}

#[test]
fn test_swap_remove_index_boundary() {
    let mut index_map: IndexMapCore<usize, usize> = IndexMapCore::new();
    index_map.push_entry(0, 1, 10);
    index_map.push_entry(1, 2, 20);
    let result = index_map.swap_remove_index(1); // this should remove the last element
}

