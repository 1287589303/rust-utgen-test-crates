// Answer 0

#[test]
fn test_decrement_indices_case1() {
    let mut indices: hash_table::HashTable<usize> = hash_table::HashTable::default();
    indices.insert(2);
    indices.insert(3);
    let mut entries: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
        Bucket { hash: HashValue(2), key: 3, value: 30 },
        Bucket { hash: HashValue(3), key: 4, value: 40 },
        Bucket { hash: HashValue(4), key: 5, value: 50 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 2);
}

#[test]
fn test_decrement_indices_case2() {
    let mut indices: hash_table::HashTable<usize> = hash_table::HashTable::default();
    indices.insert(3);
    indices.insert(4);
    let mut entries: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
        Bucket { hash: HashValue(2), key: 3, value: 30 },
        Bucket { hash: HashValue(3), key: 4, value: 40 },
        Bucket { hash: HashValue(4), key: 5, value: 50 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(1, 3);
} 

#[test]
fn test_decrement_indices_case3() {
    let mut indices: hash_table::HashTable<usize> = hash_table::HashTable::default();
    indices.insert(5);
    indices.insert(6);
    let mut entries: Vec<Bucket<i32, i32>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
        Bucket { hash: HashValue(2), key: 3, value: 30 },
        Bucket { hash: HashValue(3), key: 4, value: 40 },
        Bucket { hash: HashValue(4), key: 5, value: 50 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 5);
}

