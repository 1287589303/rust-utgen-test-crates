// Answer 0

#[test]
fn test_as_slice_non_empty() {
    struct TestSet {
        map: super::IndexMap<i32, (), std::collections::hash_map::RandomState>,
    }

    let mut test_set = TestSet {
        map: super::IndexMap::new(),
    };

    test_set.map.insert(1, ());
    test_set.map.insert(2, ());
    test_set.map.insert(3, ());

    let slice = test_set.map.as_slice();
}

#[test]
fn test_as_slice_single_entry() {
    struct TestSet {
        map: super::IndexMap<i32, (), std::collections::hash_map::RandomState>,
    }

    let mut test_set = TestSet {
        map: super::IndexMap::new(),
    };

    test_set.map.insert(42, ());

    let slice = test_set.map.as_slice();
}

#[test]
fn test_as_slice_multiple_entries() {
    struct TestSet {
        map: super::IndexMap<String, (), std::collections::hash_map::RandomState>,
    }

    let mut test_set = TestSet {
        map: super::IndexMap::new(),
    };

    test_set.map.insert("one".to_string(), ());
    test_set.map.insert("two".to_string(), ());
    test_set.map.insert("three".to_string(), ());

    let slice = test_set.map.as_slice();
}

