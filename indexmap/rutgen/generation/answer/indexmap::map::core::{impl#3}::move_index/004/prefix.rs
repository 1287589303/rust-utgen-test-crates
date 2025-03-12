// Answer 0

#[test]
fn test_move_index_equal_indices() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 0);
}

#[test]
fn test_move_index_single_element() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 0);
}

