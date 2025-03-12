// Answer 0

#[test]
fn test_get_mut_with_empty_value_and_resize() {
    struct TestStruct {
        value: usize,
    }
    
    let mut hashmap: GrowingHashmapChar<TestStruct> = GrowingHashmapChar {
        used: 0,
        fill: 2,
        mask: 7,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: TestStruct { value: 0 } },
            GrowingHashmapMapElemChar { key: 2, value: TestStruct { value: 0 } },
            GrowingHashmapMapElemChar::default(), // Default value, should trigger the condition
            GrowingHashmapMapElemChar::default(), 
            GrowingHashmapMapElemChar::default(), 
            GrowingHashmapMapElemChar::default(), 
            GrowingHashmapMapElemChar::default(), 
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    let key = 3; // A key that maps to a default value
    let result = hashmap.get_mut(key);
    
    assert_eq!(result.value, 0); // Check that we can access the value
} 

#[test]
fn test_get_mut_with_full_map_and_resize() {
    struct TestStruct {
        value: usize,
    }

    let mut hashmap: GrowingHashmapChar<TestStruct> = GrowingHashmapChar {
        used: 5,
        fill: 5,
        mask: 7,
        map: Some(vec![
            GrowingHashmapMapElemChar { key: 1, value: TestStruct { value: 1 } },
            GrowingHashmapMapElemChar { key: 2, value: TestStruct { value: 2 } },
            GrowingHashmapMapElemChar { key: 3, value: TestStruct { value: 3 } },
            GrowingHashmapMapElemChar { key: 4, value: TestStruct { value: 4 } },
            GrowingHashmapMapElemChar { key: 5, value: TestStruct { value: 5 } },
            GrowingHashmapMapElemChar::default(), // Default entry to trigger the condition
            GrowingHashmapMapElemChar::default(), 
            GrowingHashmapMapElemChar::default(),
        ]),
    };

    let key = 6; // A key that maps to a default value but will fill it
    let result = hashmap.get_mut(key);
    
    assert_eq!(result.value, 0); // Check that we can access and that it resized correctly
}

