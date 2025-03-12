// Answer 0

#[test]
fn test_get_mut_extended_ascii_upper_bound() {
    struct TestStruct {
        extended_ascii: [u32; 256],
        map: std::collections::HashMap<u32, u32>,
    }

    impl TestStruct {
        fn new() -> Self {
            Self {
                extended_ascii: [0; 256],
                map: std::collections::HashMap::new(),
            }
        }

        fn get_mut(&mut self, key: char) -> &mut u32 {
            let value = key as u32;
            if value <= 255 {
                let val_u8 = u8::try_from(value).expect("we check the bounds above");
                &mut self.extended_ascii[usize::from(val_u8)]
            } else {
                self.map.get_mut(value).expect("Key not found in map")
            }
        }
    }

    let mut test_struct = TestStruct::new();
    test_struct.extended_ascii[255] = 42; // Set a value for testing

    let result = test_struct.get_mut('\xFF'); // Test with the upper bound character

    assert_eq!(*result, 42); // Check that it returns the correct value
}

