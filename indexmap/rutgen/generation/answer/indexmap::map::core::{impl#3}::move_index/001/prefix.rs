// Answer 0

#[test]
fn test_move_index_valid_case_0_to_1() {
    let mut indices = hash_table::HashTable::default();
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: "key0", value: "value0" },
        Bucket { hash: HashValue(1), key: "key1", value: "value1" },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(0, 1);
}

#[test]
fn test_move_index_valid_case_1_to_2() {
    let mut indices = hash_table::HashTable::default();
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: "key0", value: "value0" },
        Bucket { hash: HashValue(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue(2), key: "key2", value: "value2" },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(1, 2);
}

#[test]
fn test_move_index_valid_case_2_to_3() {
    let mut indices = hash_table::HashTable::default();
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: "key0", value: "value0" },
        Bucket { hash: HashValue(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue(2), key: "key2", value: "value2" },
        Bucket { hash: HashValue(3), key: "key3", value: "value3" },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(2, 3);
}

#[test]
fn test_move_index_valid_case_3_to_4() {
    let mut indices = hash_table::HashTable::default();
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: "key0", value: "value0" },
        Bucket { hash: HashValue(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue(2), key: "key2", value: "value2" },
        Bucket { hash: HashValue(3), key: "key3", value: "value3" },
        Bucket { hash: HashValue(4), key: "key4", value: "value4" },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(3, 4);
}

#[test]
fn test_move_index_valid_case_4_to_5() {
    let mut indices = hash_table::HashTable::default();
    let mut entries = vec![
        Bucket { hash: HashValue(0), key: "key0", value: "value0" },
        Bucket { hash: HashValue(1), key: "key1", value: "value1" },
        Bucket { hash: HashValue(2), key: "key2", value: "value2" },
        Bucket { hash: HashValue(3), key: "key3", value: "value3" },
        Bucket { hash: HashValue(4), key: "key4", value: "value4" },
        Bucket { hash: HashValue(5), key: "key5", value: "value5" },
    ];
    
    let mut ref_mut = RefMut::new(&mut indices, &mut entries);
    ref_mut.move_index(4, 5);
}

