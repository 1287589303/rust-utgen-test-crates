// Answer 0

#[test]
fn test_binary_search_by_key_found() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 3],
    }

    let slice = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
        Bucket { hash: HashValue::default(), key: 2, value: 20 },
        Bucket { hash: HashValue::default(), key: 3, value: 30 },
    ]});

    let result = slice.binary_search_by_key(&2, |&entry| entry);
}

#[test]
fn test_binary_search_by_key_not_found_lesser() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 3],
    }

    let slice = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
        Bucket { hash: HashValue::default(), key: 2, value: 20 },
        Bucket { hash: HashValue::default(), key: 3, value: 30 },
    ]});

    let result = slice.binary_search_by_key(&0, |&entry| entry);
}

#[test]
fn test_binary_search_by_key_not_found_greater() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 3],
    }

    let slice = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
        Bucket { hash: HashValue::default(), key: 2, value: 20 },
        Bucket { hash: HashValue::default(), key: 3, value: 30 },
    ]});

    let result = slice.binary_search_by_key(&4, |&entry| entry);
}

#[test]
fn test_binary_search_by_key_transformation() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 3],
    }

    let slice = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
        Bucket { hash: HashValue::default(), key: 2, value: 20 },
        Bucket { hash: HashValue::default(), key: 3, value: 30 },
    ]});

    let result = slice.binary_search_by_key(&20, |&entry| entry * 10);
}

#[test]
fn test_binary_search_by_key_empty_slice() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 0],
    }

    let slice = Box::new(Slice { entries: [] });

    let result = slice.binary_search_by_key(&1, |&entry| entry);
}

#[test]
fn test_binary_search_by_key_one_element_found() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 1],
    }

    let slice = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
    ]});

    let result = slice.binary_search_by_key(&1, |&entry| entry);
}

#[test]
fn test_binary_search_by_key_two_elements_found() {
    struct TestSlice {
        entries: [Bucket<i32, i32>; 2],
    }

    let slice = Box::new(Slice { entries: [
        Bucket { hash: HashValue::default(), key: 1, value: 10 },
        Bucket { hash: HashValue::default(), key: 2, value: 20 },
    ]});

    let result = slice.binary_search_by_key(&2, |&entry| entry);
}

