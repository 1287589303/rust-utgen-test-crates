// Answer 0

#[test]
fn test_hybrid_growing_hashmap_char_default() {
    struct TestValueType;

    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType
        }
    }

    impl Clone for TestValueType {
        fn clone(&self) -> Self {
            TestValueType
        }
    }

    impl Copy for TestValueType {}

    impl PartialEq for TestValueType {
        fn eq(&self, _: &Self) -> bool {
            true
        }
    }

    let default_hybrid: HybridGrowingHashmapChar<TestValueType> = HybridGrowingHashmapChar::default();
    
    // Check if the map is default
    assert_eq!(default_hybrid.map.used, 0);
    assert_eq!(default_hybrid.map.fill, 0);
    assert_eq!(default_hybrid.map.mask, 0);
    assert!(default_hybrid.map.map.is_none());
    
    // Check if extended_ascii is filled with default values
    for value in default_hybrid.extended_ascii.iter() {
        assert_eq!(*value, TestValueType::default());
    }
}

