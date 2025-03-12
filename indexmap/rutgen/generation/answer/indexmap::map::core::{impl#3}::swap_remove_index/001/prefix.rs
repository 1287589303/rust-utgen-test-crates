// Answer 0

#[test]
fn test_swap_remove_index_valid_entry() {
    let mut indices: Indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 10 },
        Bucket { hash: HashValue(2), key: 1, value: 20 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    let index = 0;
    let result = ref_mut.swap_remove_index(index);
}

#[test]
fn test_swap_remove_index_last_entry() {
    let mut indices: Indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 10 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    let index = 0;
    let result = ref_mut.swap_remove_index(index);
}

#[test]
fn test_swap_remove_index_middle_entry() {
    let mut indices: Indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: 10 },
        Bucket { hash: HashValue(2), key: 1, value: 20 },
        Bucket { hash: HashValue(3), key: 2, value: 30 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    let index = 1;
    let result = ref_mut.swap_remove_index(index);
}

