// Answer 0

#[test]
fn test_get_mut_initial_allocation() {
    struct ValueType {
        data: i32,
    }

    let mut hashmap = GrowingHashmapChar::<ValueType> {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    // Initialize the map to trigger allocation
    let key = 5;
    let value_ref = hashmap.get_mut(key);

    // After allocation, test the returned value is mutable
    value_ref.data = 10;

    assert_eq!(hashmap.map.as_ref().unwrap()[hashmap.lookup(key)].value.data, 10);
}

#[test]
fn test_get_mut_existing_key() {
    struct ValueType {
        data: i32,
    }

    let mut hashmap = GrowingHashmapChar::<ValueType> {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    // Trigger initial allocation
    let key = 5;
    hashmap.get_mut(key).data = 3; // Set initial value for key 5

    // Call get_mut again for the existing key
    let value_ref = hashmap.get_mut(key);
    value_ref.data = 15;

    assert_eq!(hashmap.map.as_ref().unwrap()[hashmap.lookup(key)].value.data, 15);
}

#[test]
fn test_get_mut_resizing() {
    struct ValueType {
        data: i32,
    }

    // Create a hash map with a small initial size to trigger growth
    let mut hashmap = GrowingHashmapChar::<ValueType> {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    // Insert multiple items to force growth
    for key in 0..10 {
        hashmap.get_mut(key).data = key as i32;
    }

    // Verify that all values were stored correctly
    for key in 0..10 {
        assert_eq!(hashmap.map.as_ref().unwrap()[hashmap.lookup(key)].value.data, key as i32);
    }
}

