// Answer 0

#[test]
fn test_decrement_indices_boundary_case() {
    let mut indices = hash_table::HashTable::new();
    indices.insert(2, 0);
    indices.insert(3, 1);
    let mut entries = vec![
        Bucket { hash: HashValue(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue(2), key: "key2", value: "value2" },
        Bucket { hash: HashValue(3), key: "key3", value: "value3" },
        Bucket { hash: HashValue(4), key: "key4", value: "value4" },
        Bucket { hash: HashValue(5), key: "key5", value: "value5" },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(1, 4);
}

#[test]
fn test_decrement_indices_normal_case() {
    let mut indices = hash_table::HashTable::new();
    indices.insert(3, 0);
    indices.insert(4, 1);
    let mut entries = vec![
        Bucket { hash: HashValue(6), key: "key6", value: "value6" },
        Bucket { hash: HashValue(7), key: "key7", value: "value7" },
        Bucket { hash: HashValue(8), key: "key8", value: "value8" },
        Bucket { hash: HashValue(9), key: "key9", value: "value9" },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 3);
}

#[test]
fn test_decrement_indices_large_entries() {
    let mut indices = hash_table::HashTable::new();
    indices.insert(6, 0);
    indices.insert(7, 1);
    indices.insert(8, 2);
    let mut entries = vec![
        Bucket { hash: HashValue(10), key: "key10", value: "value10" },
        Bucket { hash: HashValue(11), key: "key11", value: "value11" },
        Bucket { hash: HashValue(12), key: "key12", value: "value12" },
        Bucket { hash: HashValue(13), key: "key13", value: "value13" },
        Bucket { hash: HashValue(14), key: "key14", value: "value14" },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 5);
}

#[test]
fn test_decrement_indices_shifted_entries_greater_than_half_capacity() {
    let mut indices = hash_table::HashTable::new();
    indices.insert(5, 0);
    indices.insert(6, 1);
    indices.insert(7, 2);
    let mut entries = vec![
        Bucket { hash: HashValue(15), key: "key15", value: "value15" },
        Bucket { hash: HashValue(16), key: "key16", value: "value16" },
        Bucket { hash: HashValue(17), key: "key17", value: "value17" },
        Bucket { hash: HashValue(18), key: "key18", value: "value18" },
        Bucket { hash: HashValue(19), key: "key19", value: "value19" },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.decrement_indices(0, 4);
}

