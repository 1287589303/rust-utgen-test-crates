// Answer 0

#[test]
fn test_index_mut_existing_key() {
    struct TestMap {
        map: crate::IndexMap<String, i32, crate::RandomState>,
    }
    let mut test_map = TestMap {
        map: crate::IndexMap::new(),
    };
    test_map.map.insert("key1".to_string(), 10);
    let value = test_map.map.index_mut(&"key1".to_string());
}

#[test]
#[should_panic(expected = "no entry found for key")]
fn test_index_mut_non_existing_key() {
    struct TestMap {
        map: crate::IndexMap<String, i32, crate::RandomState>,
    }
    let mut test_map = TestMap {
        map: crate::IndexMap::new(),
    };
    test_map.map.index_mut(&"non_existing_key".to_string());
}

#[test]
fn test_index_mut_empty_map() {
    struct TestMap {
        map: crate::IndexMap<String, i32, crate::RandomState>,
    }
    let mut test_map = TestMap {
        map: crate::IndexMap::new(),
    };
    test_map.map.insert("key2".to_string(), 20);
    let value = test_map.map.index_mut(&"key2".to_string());
}

#[test]
fn test_index_mut_edge_case() {
    struct TestMap {
        map: crate::IndexMap<i32, i32, crate::RandomState>,
    }
    let mut test_map = TestMap {
        map: crate::IndexMap::new(),
    };
    test_map.map.insert(0, 100);
    let value = test_map.map.index_mut(&0);
}

