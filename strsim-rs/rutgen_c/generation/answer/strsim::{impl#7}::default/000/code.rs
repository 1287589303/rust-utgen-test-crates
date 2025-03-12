// Answer 0

#[test]
fn test_hybrid_growing_hashmap_char_default() {
    struct TestValueType;
    impl Default for TestValueType {
        fn default() -> Self {
            TestValueType
        }
    }
    
    let hashmap: HybridGrowingHashmapChar<TestValueType> = HybridGrowingHashmapChar::default();
    
    // Check that the map is initialized to default
    assert_eq!(hashmap.map.used, 0);
    assert_eq!(hashmap.map.fill, 0);
    assert_eq!(hashmap.map.mask, 0);
    assert!(hashmap.map.map.is_none());
    
    // Check that the extended ASCII array is initialized with default values
    for value in &hashmap.extended_ascii {
        assert_eq!(*value, TestValueType::default());
    }
}

