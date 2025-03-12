// Answer 0

#[test]
fn test_get_mut_new_entry() {
    struct TestValue {
        value: i32,
    }
    
    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    
    let key: u32 = 42;
    let value_ref = hashmap.get_mut(key);
    value_ref.value = 100;

    assert_eq!(hashmap.used, 1);
    assert_eq!(hashmap.fill, 1);
    assert_eq!(hashmap.map.as_ref().unwrap()[hashmap.lookup(key)].value.value, 100);
}

#[test]
fn test_get_mut_existing_entry() {
    struct TestValue {
        value: i32,
    }
    
    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    
    let key: u32 = 42;
    let value_ref = hashmap.get_mut(key);
    value_ref.value = 100;

    // Update the existing entry
    let value_ref_existing = hashmap.get_mut(key);
    value_ref_existing.value = 200;

    assert_eq!(hashmap.used, 1);
    assert_eq!(hashmap.fill, 1);
    assert_eq!(hashmap.map.as_ref().unwrap()[hashmap.lookup(key)].value.value, 200);
}

#[test]
fn test_get_mut_growth() {
    struct TestValue {
        value: i32,
    }
    
    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };

    // Fill the hashmap to trigger growth
    for i in 0..6 {
        let value_ref = hashmap.get_mut(i);
        value_ref.value = i as i32 * 10;
    }

    assert!(hashmap.map.is_some());
    assert!(hashmap.mask > 0);
    assert_eq!(hashmap.used, 6);
}

#[test]
fn test_get_mut_initialization() {
    struct TestValue {
        value: i32,
    }
    
    let mut hashmap: GrowingHashmapChar<TestValue> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    
    assert!(hashmap.map.is_none());

    let value_ref = hashmap.get_mut(1);
    value_ref.value = 50;

    assert!(hashmap.map.is_some());
    assert_eq!(hashmap.used, 1);
    assert_eq!(hashmap.fill, 1);
    assert_eq!(hashmap.map.as_ref().unwrap()[hashmap.lookup(1)].value.value, 50);
}

