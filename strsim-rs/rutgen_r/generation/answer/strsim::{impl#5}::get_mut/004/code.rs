// Answer 0

#[test]
fn test_get_mut_with_map_initialized_and_value_default() {
    struct ValueType {
        value: i32,
        key: u32,
    }

    struct TestStruct {
        map: Option<Vec<ValueType>>,
        fill: usize,
        used: usize,
        mask: usize,
    }

    impl TestStruct {
        fn allocate(&mut self) {
            self.map = Some(vec![ValueType { value: Default::default(), key: 0 }; 8]);
        }

        fn lookup(&self, key: u32) -> usize {
            key as usize % self.map.as_ref().unwrap().len()
        }

        fn grow(&mut self, _new_size: usize) {
            // Simulate growth by updating the mask (for the sake of the test)
            self.mask *= 2;
        }

        fn get_mut(&mut self, key: u32) -> &mut i32 {
            if self.map.is_none() {
                self.allocate();
            }

            let mut i = self.lookup(key);
            if self.map.as_ref().expect("map should have been created above")[i].value == Default::default() {
                self.fill += 1;
                // resize when 2/3 full
                if self.fill * 3 >= (self.mask + 1) * 2 {
                    self.grow((self.used + 1) * 2);
                    i = self.lookup(key);
                }

                self.used += 1;
            }

            let elem = &mut self.map.as_mut().expect("map should have been created above")[i];
            elem.key = key;
            &mut elem.value
        }
    }

    let mut test_struct = TestStruct {
        map: Some(vec![ValueType { value: Default::default(), key: 0 }; 8]),
        fill: 4,
        used: 4,
        mask: 7,
    };

    let value = test_struct.get_mut(4);
    *value = 42; // Set the value to verify it's mutable

    assert_eq!(test_struct.map.as_ref().unwrap()[test_struct.lookup(4)].value, 42);
}

