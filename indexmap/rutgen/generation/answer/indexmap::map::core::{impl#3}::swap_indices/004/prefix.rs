// Answer 0

#[test]
fn test_swap_indices_equal_in_bounds() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(11), key: 0, value: 10 },
        Bucket { hash: HashValue(22), key: 1, value: 20 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(0, 0);
}

#[test]
fn test_swap_indices_equal_out_of_bounds() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(11), key: 0, value: 10 },
        Bucket { hash: HashValue(22), key: 1, value: 20 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(2, 2);
}

#[test]
fn test_swap_indices_valid_hashes() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(11), key: 0, value: 10 },
        Bucket { hash: HashValue(22), key: 1, value: 20 },
    ];
    
    indices.insert(11.get(), 0);
    indices.insert(22.get(), 1);
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.swap_indices(0, 1);
}

