// Answer 0

#[test]
fn test_truncate_non_empty_map() {
    struct TestEntry {
        hash: HashValue,
        key: usize,
        value: usize,
    }

    let mut map = IndexMapCore::new();
    for i in 0..5 {
        map.push_entry(HashValue::from(i), i, i);
    }
    let current_len = map.len();

    map.truncate(current_len - 1);
}

#[test]
fn test_truncate_with_one_entry() {
    struct TestEntry {
        hash: HashValue,
        key: usize,
        value: usize,
    }

    let mut map = IndexMapCore::new();
    for i in 0..3 {
        map.push_entry(HashValue::from(i), i, i);
    }
    let current_len = map.len();

    map.truncate(current_len - 1);
}

#[test]
fn test_truncate_boundary_case() {
    struct TestEntry {
        hash: HashValue,
        key: usize,
        value: usize,
    }

    let mut map = IndexMapCore::new();
    for i in 0..10 {
        map.push_entry(HashValue::from(i), i, i);
    }
    let current_len = map.len();

    map.truncate(current_len - 2);
}

