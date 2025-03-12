// Answer 0

#[test]
fn test_insert_unique_with_less_entries_than_capacity() {
    let mut indices: Indices = Indices::new();
    let mut entries: Vec<Bucket<usize, String>> = Vec::with_capacity(4);
    let ref_mut = RefMut::new(&mut indices, &mut entries);

    let hash = HashValue(1);
    let key = 10;
    let value = String::from("value1");
    let _occupied_entry = ref_mut.insert_unique(hash, key, value);

    let hash2 = HashValue(2);
    let key2 = 20;
    let value2 = String::from("value2");
    let _occupied_entry2 = ref_mut.insert_unique(hash2, key2, value2);
}

#[test]
fn test_insert_unique_with_multiple_entries() {
    let mut indices: Indices = Indices::new();
    let mut entries: Vec<Bucket<usize, String>> = Vec::with_capacity(10);
    let ref_mut = RefMut::new(&mut indices, &mut entries);

    let hash = HashValue(3);
    let key = 30;
    let value = String::from("value3");
    let _occupied_entry = ref_mut.insert_unique(hash, key, value);

    let hash2 = HashValue(4);
    let key2 = 40;
    let value2 = String::from("value4");
    let _occupied_entry2 = ref_mut.insert_unique(hash2, key2, value2);

    let hash3 = HashValue(5);
    let key3 = 50;
    let value3 = String::from("value5");
    let _occupied_entry3 = ref_mut.insert_unique(hash3, key3, value3);
}

#[test]
fn test_insert_unique_with_high_hash_value() {
    let mut indices: Indices = Indices::new();
    let mut entries: Vec<Bucket<usize, String>> = Vec::with_capacity(5);
    let ref_mut = RefMut::new(&mut indices, &mut entries);

    let hash = HashValue(100);
    let key = 60;
    let value = String::from("value6");
    let _occupied_entry = ref_mut.insert_unique(hash, key, value);

    let hash2 = HashValue(101);
    let key2 = 70;
    let value2 = String::from("value7");
    let _occupied_entry2 = ref_mut.insert_unique(hash2, key2, value2);
}

