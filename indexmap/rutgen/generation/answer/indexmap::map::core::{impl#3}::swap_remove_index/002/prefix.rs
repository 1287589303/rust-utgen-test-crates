// Answer 0

#[test]
fn test_swap_remove_index_out_of_bounds_negative() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let index = usize::MAX; // Out of bounds
    let result = ref_mut.swap_remove_index(index);
}

#[test]
fn test_swap_remove_index_out_of_bounds_too_large() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let index = entries.len(); // Index equals length of entries, which is out of bounds
    let result = ref_mut.swap_remove_index(index);
}

