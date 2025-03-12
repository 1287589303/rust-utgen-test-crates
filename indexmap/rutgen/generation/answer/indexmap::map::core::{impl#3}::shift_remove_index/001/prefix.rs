// Answer 0

#[test]
fn test_shift_remove_index_valid_case() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
        Bucket { hash: HashValue(2), key: 3, value: 30 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let index = 1; // Valid index
    let result = ref_mut.shift_remove_index(index);
}

#[test]
fn test_shift_remove_index_first_entry() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let index = 0; // Valid index (first entry)
    let result = ref_mut.shift_remove_index(index);
}

#[test]
fn test_shift_remove_index_last_entry() {
    let mut indices = hash_table::HashTable::default();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(0), key: 1, value: 10 },
        Bucket { hash: HashValue(1), key: 2, value: 20 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let index = 1; // Valid index (last entry)
    let result = ref_mut.shift_remove_index(index);
}

