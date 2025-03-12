// Answer 0

#[test]
fn test_binary_search_by_key_found_first() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 0, key: 2, value: "b" },
        Bucket { hash: 0, key: 3, value: "c" },
    ] });
    let result = slice.binary_search_by_key(&1, |k, _| *k);
}

#[test]
fn test_binary_search_by_key_found_last() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 0, key: 2, value: "b" },
        Bucket { hash: 0, key: 3, value: "c" },
    ] });
    let result = slice.binary_search_by_key(&3, |k, _| *k);
}

#[test]
fn test_binary_search_by_key_found_middle() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 0, key: 2, value: "b" },
        Bucket { hash: 0, key: 3, value: "c" },
    ] });
    let result = slice.binary_search_by_key(&2, |k, _| *k);
}

#[test]
fn test_binary_search_by_key_not_found_less_than_first() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 0, key: 2, value: "b" },
        Bucket { hash: 0, key: 3, value: "c" },
    ] });
    let result = slice.binary_search_by_key(&0, |k, _| *k);
}

#[test]
fn test_binary_search_by_key_not_found_greater_than_last() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 0, key: 2, value: "b" },
        Bucket { hash: 0, key: 3, value: "c" },
    ] });
    let result = slice.binary_search_by_key(&4, |k, _| *k);
}

#[test]
fn test_binary_search_by_key_not_found_between() {
    let slice = Box::new(Slice { entries: [
        Bucket { hash: 0, key: 1, value: "a" },
        Bucket { hash: 0, key: 2, value: "b" },
        Bucket { hash: 0, key: 3, value: "c" },
    ] });
    let result = slice.binary_search_by_key(&2.5, |k, _| *k as f64);
}

