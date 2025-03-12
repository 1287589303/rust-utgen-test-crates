// Answer 0

#[test]
fn test_decrement_indices_empty_entries() {
    let mut indices: Indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 0);
}

#[test]
fn test_decrement_indices_half_capacity() {
    let mut indices: Indices = hash_table::HashTable::with_capacity(100);
    let mut entries: Vec<Bucket<usize, usize>> = vec![Bucket { hash: HashValue(1), key: 0, value: 0 }; 50];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 50);
}

#[test]
fn test_decrement_indices_full_capacity() {
    let mut indices: Indices = hash_table::HashTable::with_capacity(100);
    let mut entries: Vec<Bucket<usize, usize>> = vec![Bucket { hash: HashValue(1), key: 0, value: 0 }; 50];

    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 50);
}

