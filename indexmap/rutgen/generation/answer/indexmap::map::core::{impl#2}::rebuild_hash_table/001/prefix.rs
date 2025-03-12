// Answer 0

#[test]
fn test_rebuild_hash_table_with_valid_entries() {
    struct TestBucket {
        hash: HashValue,
        key: usize,
        value: usize,
    }
    
    let entry1 = TestBucket { hash: HashValue::new(10), key: 1, value: 100 };
    let entry2 = TestBucket { hash: HashValue::new(20), key: 2, value: 200 };
    
    let mut index_map = IndexMapCore::<usize, usize>::with_capacity(2);
    index_map.entries.push(Bucket { hash: entry1.hash, key: entry1.key, value: entry1.value });
    index_map.entries.push(Bucket { hash: entry2.hash, key: entry2.key, value: entry2.value });
    
    index_map.rebuild_hash_table();
}

#[test]
fn test_rebuild_hash_table_with_minimal_entries() {
    struct TestBucket {
        hash: HashValue,
        key: usize,
        value: usize,
    }
    
    let entry = TestBucket { hash: HashValue::new(10), key: 1, value: 100 };
    
    let mut index_map = IndexMapCore::<usize, usize>::with_capacity(1);
    index_map.entries.push(Bucket { hash: entry.hash, key: entry.key, value: entry.value });
    
    index_map.rebuild_hash_table();
}

#[test]
fn test_rebuild_hash_table_with_multiple_entries() {
    struct TestBucket {
        hash: HashValue,
        key: usize,
        value: usize,
    }
    
    let entry1 = TestBucket { hash: HashValue::new(10), key: 1, value: 100 };
    let entry2 = TestBucket { hash: HashValue::new(20), key: 2, value: 200 };
    let entry3 = TestBucket { hash: HashValue::new(30), key: 3, value: 300 };
    
    let mut index_map = IndexMapCore::<usize, usize>::with_capacity(3);
    index_map.entries.push(Bucket { hash: entry1.hash, key: entry1.key, value: entry1.value });
    index_map.entries.push(Bucket { hash: entry2.hash, key: entry2.key, value: entry2.value });
    index_map.entries.push(Bucket { hash: entry3.hash, key: entry3.key, value: entry3.value });
    
    index_map.rebuild_hash_table();
}

#[test]
fn test_rebuild_hash_table_with_exact_capacity() {
    struct TestBucket {
        hash: HashValue,
        key: usize,
        value: usize,
    }
    
    let entry1 = TestBucket { hash: HashValue::new(10), key: 1, value: 100 };
    
    let mut index_map = IndexMapCore::<usize, usize>::with_capacity(1);
    index_map.entries.push(Bucket { hash: entry1.hash, key: entry1.key, value: entry1.value });
    
    index_map.rebuild_hash_table();
}

