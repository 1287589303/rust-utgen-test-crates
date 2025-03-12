// Answer 0

#[test]
fn test_get_mut_extended_ascii_bound() {
    struct MockValueType {
        value: usize,
    }

    impl Default for MockValueType {
        fn default() -> Self {
            MockValueType { value: 0 }
        }
    }

    let mut hashmap = HybridGrowingHashmapChar {
        map: GrowingHashmapChar {
            used: 0,
            fill: 0,
            mask: 0,
            map: None,
        },
        extended_ascii: [MockValueType::default(); 256],
    };

    let key: char = char::from(255); // Testing the boundary at 255
    let value = hashmap.get_mut(key);
    value.value = 42; // Assigning a value to check if it is set correctly
    
    assert_eq!(hashmap.extended_ascii[255].value, 42);
}

#[test]
fn test_get_mut_extended_ascii_lower_bound() {
    struct MockValueType {
        value: usize,
    }

    impl Default for MockValueType {
        fn default() -> Self {
            MockValueType { value: 0 }
        }
    }
    
    let mut hashmap = HybridGrowingHashmapChar {
        map: GrowingHashmapChar {
            used: 0,
            fill: 0,
            mask: 0,
            map: None,
        },
        extended_ascii: [MockValueType::default(); 256],
    };

    let key: char = char::from(0); // Testing lower boundary
    let value = hashmap.get_mut(key);
    value.value = 13; // Assigning a value to check if it is set correctly

    assert_eq!(hashmap.extended_ascii[0].value, 13);
}

