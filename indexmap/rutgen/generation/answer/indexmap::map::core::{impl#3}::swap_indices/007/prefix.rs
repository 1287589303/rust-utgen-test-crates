// Answer 0

#[test]
fn test_swap_indices_different_elements() {
    let mut indices = hash_table::HashTable::new();
    let hash_value_a = HashValue(1);
    let hash_value_b = HashValue(2);
    let entry_a = Bucket { hash: hash_value_a, key: 0, value: "a" };
    let entry_b = Bucket { hash: hash_value_b, key: 1, value: "b" };
    let mut entries = vec![entry_a, entry_b];
    indices.insert(hash_value_a.get(), 0);
    indices.insert(hash_value_b.get(), 1);
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(0, 1);
}

#[test]
fn test_swap_indices_with_same_hash() {
    let mut indices = hash_table::HashTable::new();
    let hash_value = HashValue(1);
    let entry_a = Bucket { hash: hash_value, key: 0, value: "a" };
    let entry_b = Bucket { hash: hash_value, key: 1, value: "b" };
    let mut entries = vec![entry_a, entry_b];
    indices.insert(hash_value.get(), 0);
    indices.insert(hash_value.get(), 1);
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(0, 1);
}

