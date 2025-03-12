// Answer 0

#[test]
fn test_sort_by_cached_key_with_multiple_entries() {
    struct TestMap {
        data: indexmap::IndexMap<i32, String>,
    }

    let mut test_map = TestMap {
        data: indexmap::IndexMap::new(),
    };

    test_map.data.insert(3, "three".to_string());
    test_map.data.insert(1, "one".to_string());
    test_map.data.insert(2, "two".to_string());

    test_map.data.sort_by_cached_key(|k, v| (v.len(), *k));
}

#[test]
fn test_sort_by_cached_key_with_single_entry() {
    struct TestMap {
        data: indexmap::IndexMap<i32, String>,
    }

    let mut test_map = TestMap {
        data: indexmap::IndexMap::new(),
    };

    test_map.data.insert(5, "five".to_string());

    test_map.data.sort_by_cached_key(|k, v| (v.len(), *k));
}

#[test]
fn test_sort_by_cached_key_with_empty_map() {
    struct TestMap {
        data: indexmap::IndexMap<i32, String>,
    }

    let mut test_map = TestMap {
        data: indexmap::IndexMap::new(),
    };

    test_map.data.sort_by_cached_key(|k, v| (v.len(), *k));
} 

#[test]
fn test_sort_by_cached_key_with_identical_keys() {
    struct TestMap {
        data: indexmap::IndexMap<i32, String>,
    }

    let mut test_map = TestMap {
        data: indexmap::IndexMap::new(),
    };

    test_map.data.insert(1, "one".to_string());
    test_map.data.insert(1, "uno".to_string());
    test_map.data.insert(1, "ein".to_string());

    test_map.data.sort_by_cached_key(|k, v| (v.len(), *k));
} 

#[test]
fn test_sort_by_cached_key_with_reversed_order() {
    struct TestMap {
        data: indexmap::IndexMap<i32, String>,
    }

    let mut test_map = TestMap {
        data: indexmap::IndexMap::new(),
    };

    test_map.data.insert(5, "five".to_string());
    test_map.data.insert(4, "four".to_string());
    test_map.data.insert(3, "three".to_string());

    test_map.data.sort_by_cached_key(|k, v| (v.len(), *k));
}

