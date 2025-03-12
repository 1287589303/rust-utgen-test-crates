// Answer 0

#[test]
fn test_swap_remove_full_non_empty() {
    struct TestMap {
        map: crate::IndexMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                map: crate::IndexMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.map.insert(key, value);
        }
    }

    let mut test_map = TestMap::new();
    test_map.insert(1, String::from("value1"));
    test_map.insert(2, String::from("value2"));

    let (index, value) = test_map.map.swap_remove_full(&1).unwrap();
}

#[test]
fn test_swap_remove_full_multiple_elements() {
    struct TestMap {
        map: crate::IndexMap<i32, String>,
    }

    impl TestMap {
        fn new() -> Self {
            TestMap {
                map: crate::IndexMap::new(),
            }
        }

        fn insert(&mut self, key: i32, value: String) {
            self.map.insert(key, value);
        }
    }

    let mut test_map = TestMap::new();
    test_map.insert(3, String::from("value3"));
    test_map.insert(4, String::from("value4"));

    let (index, value) = test_map.map.swap_remove_full(&3).unwrap();
}

