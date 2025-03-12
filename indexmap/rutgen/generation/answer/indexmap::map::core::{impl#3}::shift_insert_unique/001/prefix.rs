// Answer 0

#[test]
fn test_shift_insert_unique_at_end() {
    let mut indices = hash_table::HashTable::<usize>::default();
    let mut entries: Vec<Bucket<i32, String>> = Vec::new();
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // Prepare conditions with index == end
    let index = ref_mut.indices.len();
    let hash = HashValue(42);
    let key = 1;
    let value = String::from("value");

    ref_mut.shift_insert_unique(index, hash, key, value);
}

#[test]
fn test_shift_insert_unique_at_middle() {
    let mut indices = hash_table::HashTable::<usize>::default();
    let mut entries: Vec<Bucket<i32, String>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: String::from("zero") },
        Bucket { hash: HashValue(2), key: 1, value: String::from("one") },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // Prepare conditions with index <= end
    let index = 1; // Inserting at index 1 that is within the current length
    let hash = HashValue(3);
    let key = 2;
    let value = String::from("two");

    ref_mut.shift_insert_unique(index, hash, key, value);
}

#[test]
fn test_shift_insert_unique_when_capacity_reached() {
    let mut indices = hash_table::HashTable::<usize>::default();
    let mut entries: Vec<Bucket<i32, String>> = vec![
        Bucket { hash: HashValue(1), key: 0, value: String::from("zero") },
        Bucket { hash: HashValue(2), key: 1, value: String::from("one") },
    ];
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);

    // Prepare conditions with length equal to capacity
    let index = 0; // Inserting at index 0, which is valid as entries has a capacity
    let hash = HashValue(3);
    let key = 3;
    let value = String::from("three");

    ref_mut.shift_insert_unique(index, hash, key, value);
}

