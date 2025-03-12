// Answer 0

#[test]
fn test_get_extended_ascii_boundary_value() {
    struct TestStruct {
        extended_ascii: [u8; 256],
        map: std::collections::HashMap<u32, u8>,
    }

    impl TestStruct {
        fn new() -> Self {
            let mut extended_ascii = [0u8; 256];
            for i in 0..=255 {
                extended_ascii[i as usize] = i as u8;
            }
            TestStruct {
                extended_ascii,
                map: std::collections::HashMap::new(),
            }
        }

        fn get(&self, key: char) -> u8 {
            let value = key as u32;
            if value <= 255 {
                let val_u8 = u8::try_from(value).expect("we check the bounds above");
                self.extended_ascii[usize::from(val_u8)]
            } else {
                *self.map.get(&value).unwrap_or(&0)
            }
        }
    }

    let test_struct = TestStruct::new();
    let key = 'ÿ'; // Unicode character with value 255
    let expected_value = 255; // Since we initialized extended_ascii with its indices
    let result = test_struct.get(key);
    assert_eq!(result, expected_value);
}

