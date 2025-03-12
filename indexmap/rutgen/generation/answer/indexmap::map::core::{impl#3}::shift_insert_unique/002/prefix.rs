// Answer 0

#[test]
fn test_shift_insert_unique_bound_index() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let index = 0;
    let hash = HashValue(42);
    let key = 1;
    let value = 10;
    ref_mut.shift_insert_unique(index, hash, key, value);
}

#[test]
fn test_shift_insert_unique_multiple_entries() {
    let mut indices = hash_table::HashTable::new();
    let mut entries: Vec<Bucket<usize, usize>> = vec![
        Bucket { hash: HashValue(1), key: 2, value: 20 },
        Bucket { hash: HashValue(2), key: 3, value: 30 },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let index = 2; // valid position equal to current length
    let hash = HashValue(42);
    let key = 4;
    let value = 40;
    ref_mut.shift_insert_unique(index, hash, key, value);
}

#[test]
fn test_shift_insert_unique_no_capacity() {
    let mut indices = hash_table::HashTable::new();
    let capacity = 5;
    let mut entries: Vec<Bucket<usize, usize>> = Vec::with_capacity(capacity);
    for i in 0..capacity {
        entries.push(Bucket { hash: HashValue(i as usize), key: i, value: i * 10 });
    }
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    let index = entries.len(); // bound case where index == capacity
    let hash = HashValue(99);
    let key = 6;
    let value = 60;
    ref_mut.shift_insert_unique(index, hash, key, value);
}

