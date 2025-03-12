// Answer 0

#[test]
fn test_decrement_indices_case1() {
    let mut indices: Indices = hash_table::HashTable::new();
    indices.extend(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let mut entries: Entries<usize, usize> = vec![
        Bucket { hash: HashValue(0), key: 0, value: 0 },
        Bucket { hash: HashValue(1), key: 1, value: 1 },
        Bucket { hash: HashValue(2), key: 2, value: 2 },
        Bucket { hash: HashValue(3), key: 3, value: 3 },
        Bucket { hash: HashValue(4), key: 4, value: 4 },
        Bucket { hash: HashValue(5), key: 5, value: 5 },
        Bucket { hash: HashValue(6), key: 6, value: 6 },
        Bucket { hash: HashValue(7), key: 7, value: 7 },
        Bucket { hash: HashValue(8), key: 8, value: 8 },
        Bucket { hash: HashValue(9), key: 9, value: 9 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 5);
}

#[test]
fn test_decrement_indices_case2() {
    let mut indices: Indices = hash_table::HashTable::new();
    indices.extend(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 10]);
    let mut entries: Entries<usize, usize> = vec![
        Bucket { hash: HashValue(0), key: 0, value: 0 },
        Bucket { hash: HashValue(1), key: 1, value: 1 },
        Bucket { hash: HashValue(2), key: 2, value: 2 },
        Bucket { hash: HashValue(3), key: 3, value: 3 },
        Bucket { hash: HashValue(4), key: 4, value: 4 },
        Bucket { hash: HashValue(5), key: 5, value: 5 },
        Bucket { hash: HashValue(6), key: 6, value: 6 },
        Bucket { hash: HashValue(7), key: 7, value: 7 },
        Bucket { hash: HashValue(8), key: 8, value: 8 },
        Bucket { hash: HashValue(9), key: 9, value: 9 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(5, 10);
}

