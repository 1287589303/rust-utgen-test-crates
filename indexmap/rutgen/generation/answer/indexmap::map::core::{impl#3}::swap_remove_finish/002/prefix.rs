// Answer 0

#[test]
fn test_swap_remove_finish_valid_index() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    let result = ref_mut.swap_remove_finish(1);
}

#[test]
fn test_swap_remove_finish_first_index() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    let result = ref_mut.swap_remove_finish(0);
}

#[test]
fn test_swap_remove_finish_last_index() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    let result = ref_mut.swap_remove_finish(2);
}

#[test]
fn test_swap_remove_finish_middle_index() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 1, value: 10 },
        Bucket { hash: HashValue(2), key: 2, value: 20 },
        Bucket { hash: HashValue(3), key: 3, value: 30 },
        Bucket { hash: HashValue(4), key: 4, value: 40 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    let result = ref_mut.swap_remove_finish(1);
}

