// Answer 0

#[test]
fn test_split_off_empty() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::new();
    let result = map.split_off(0);
}

#[test]
fn test_split_off_full() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(5);
    for i in 0..5 {
        map.entries.push(Bucket {
            hash: HashValue::from(i as u64),
            key: i,
            value: i * 10,
        });
    }
    let result = map.split_off(5);
}

#[test]
fn test_split_off_one_element() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(2);
    map.entries.push(Bucket {
        hash: HashValue::from(1),
        key: 1,
        value: 10,
    });
    let result = map.split_off(1);
}

#[test]
fn test_split_off_middle_element() {
    let mut map: IndexMapCore<usize, usize> = IndexMapCore::with_capacity(3);
    for i in 0..3 {
        map.entries.push(Bucket {
            hash: HashValue::from(i as u64),
            key: i,
            value: i * 10,
        });
    }
    let result = map.split_off(1);
}

