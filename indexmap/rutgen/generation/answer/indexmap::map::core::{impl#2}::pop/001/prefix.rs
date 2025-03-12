// Answer 0

#[test]
fn test_pop_single_element() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 10 });
    let result = map.pop();
}

#[test]
fn test_pop_multiple_elements() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::new();
    map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 10 });
    map.entries.push(Bucket { hash: HashValue(2), key: 2, value: 20 });
    let result = map.pop();
}

#[test]
fn test_pop_on_non_empty_map() {
    let mut map: IndexMapCore<i32, i32> = IndexMapCore::with_capacity(5);
    map.entries.push(Bucket { hash: HashValue(1), key: 1, value: 10 });
    let result = map.pop();
}

