// Answer 0

#[test]
fn test_allocate_initializes_mask_and_map() {
    struct TestValueType;
    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType
        }
    }
    impl PartialEq for TestValueType {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }
    
    let mut hashmap = GrowingHashmapChar::<TestValueType> {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    hashmap.allocate();

    assert_eq!(hashmap.mask, 7);
    assert!(hashmap.map.is_some());
    assert_eq!(hashmap.map.as_ref().unwrap().len(), 8);
}

#[test]
fn test_allocate_empty_map() {
    struct TestValueType;
    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType
        }
    }
    
    let mut hashmap = GrowingHashmapChar::<TestValueType> {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    hashmap.allocate();

    assert_eq!(hashmap.map.as_ref().unwrap().iter().filter(|elem| elem.value == TestValueType).count(), 8);
}

#[test]
fn test_allocate_does_not_modify_other_fields() {
    struct TestValueType;
    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType
        }
    }
    
    let mut hashmap = GrowingHashmapChar::<TestValueType> {
        used: 5,
        fill: 3,
        mask: 2,
        map: None,
    };

    hashmap.allocate();

    assert_eq!(hashmap.used, 5);
    assert_eq!(hashmap.fill, 3);
}

