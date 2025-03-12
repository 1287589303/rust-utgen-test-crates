// Answer 0

#[test]
fn test_get_mut_with_extended_ascii() {
    struct ValueType {
        data: i32,
    }
  
    struct TestStruct {
        extended_ascii: Vec<ValueType>,
        map: std::collections::HashMap<u32, ValueType>,
    }

    let mut test_struct = TestStruct {
        extended_ascii: vec![ValueType { data: 0 }; 256], // initialize with 256 elements
        map: std::collections::HashMap::new(),
    };

    let key: char = 'A'; // ASCII character
    let value_type: &mut ValueType = test_struct.get_mut(key);
    value_type.data = 42; // mutate the value

    assert_eq!(test_struct.extended_ascii[key as usize as usize].data, 42);
}

#[test]
fn test_get_mut_with_non_ascii() {
    struct ValueType {
        data: i32,
    }

    struct TestStruct {
        extended_ascii: Vec<ValueType>,
        map: std::collections::HashMap<u32, ValueType>,
    }

    let mut test_struct = TestStruct {
        extended_ascii: vec![ValueType { data: 0 }; 256],
        map: std::collections::HashMap::new(),
    };

    let key: char = '€'; // Non-ASCII character
    test_struct.map.insert(key as u32, ValueType { data: 100 });
    let value_type: &mut ValueType = test_struct.get_mut(key);
    value_type.data = 200; // mutate the value

    assert_eq!(test_struct.map.get(&key as u32).unwrap().data, 200);
}

