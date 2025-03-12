// Answer 0

#[test]
fn test_split_last_single_element() {
    let bucket = Bucket { hash: 0, key: "key1", value: "value1" };
    let entries = vec![bucket];
    let slice = Box::new(Slice { entries: entries.try_into().unwrap() });
    
    let result = slice.split_last();
}

#[test]
fn test_split_last_two_elements() {
    let bucket1 = Bucket { hash: 0, key: "key1", value: "value1" };
    let bucket2 = Bucket { hash: 1, key: "key2", value: "value2" };
    let entries = vec![bucket1, bucket2];
    let slice = Box::new(Slice { entries: entries.try_into().unwrap() });
    
    let result = slice.split_last();
}

#[test]
fn test_split_last_multiple_elements() {
    let bucket1 = Bucket { hash: 0, key: "key1", value: "value1" };
    let bucket2 = Bucket { hash: 1, key: "key2", value: "value2" };
    let bucket3 = Bucket { hash: 2, key: "key3", value: "value3" };
    let entries = vec![bucket1, bucket2, bucket3];
    let slice = Box::new(Slice { entries: entries.try_into().unwrap() });
    
    let result = slice.split_last();
}

