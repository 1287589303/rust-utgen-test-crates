// Answer 0

#[test]
fn test_swap_indices_equal_in_bounds() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    let a = 0;
    let b = 0;

    ref_mut.swap_indices(a, b);
}

#[test]
fn test_swap_indices_equal_at_max_index() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
        Bucket { hash: HashValue(2), key: 3, value: 30 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    let a = 2;
    let b = 2;

    ref_mut.swap_indices(a, b);
}

#[test]
fn test_swap_indices_equal_with_multiple_entries() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
        Bucket { hash: HashValue(2), key: 3, value: 30 },
        Bucket { hash: HashValue(3), key: 4, value: 40 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    
    let a = 1;
    let b = 1;

    ref_mut.swap_indices(a, b);
}

