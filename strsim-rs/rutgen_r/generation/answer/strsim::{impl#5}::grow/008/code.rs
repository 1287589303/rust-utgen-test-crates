// Answer 0

#[test]
fn test_grow_with_new_size_greater_than_min_used_and_no_elements_in_old_map() {
    struct GrowingHashmapMapElemChar {
        key: char,
        value: i32,
    }

    struct HashMap {
        mask: i32,
        used: i32,
        fill: i32,
        map: Option<Vec<GrowingHashmapMapElemChar>>,
    }

    impl HashMap {
        fn new() -> Self {
            Self {
                mask: 0,
                used: 0,
                fill: 0,
                map: Some(vec![GrowingHashmapMapElemChar { key: '\0', value: 0 }; 1]),
            }
        }

        fn lookup(&self, _key: char) -> usize {
            0 // Simplified for the test
        }

        fn grow(&mut self, min_used: i32) {
            let mut new_size = self.mask + 1;
            while new_size <= min_used {
                new_size <<= 1;
            }

            self.fill = self.used;
            self.mask = new_size - 1;

            let old_map = std::mem::replace(
                self.map
                    .as_mut()
                    .expect("callers have to ensure map is allocated"),
                vec![GrowingHashmapMapElemChar::default(); new_size as usize],
            );

            for elem in old_map {
                if elem.value != Default::default() {
                    let j = self.lookup(elem.key);
                    let new_elem = &mut self.map.as_mut().expect("map created above")[j];
                    new_elem.key = elem.key;
                    new_elem.value = elem.value;
                    self.used -= 1;
                    if self.used == 0 {
                        break;
                    }
                }
            }

            self.used = self.fill;
        }
    }

    let mut hashmap = HashMap::new();
    hashmap.mask = 1; // Initialize mask to allow new_size to be greater than min_used
    hashmap.used = 0; // No elements used

    hashmap.grow(1); // Ensure min_used is less than new_size
    assert_eq!(hashmap.mask, 1); // mask should be set accordingly
    assert_eq!(hashmap.fill, 0); // fill should reflect used elements
    assert!(hashmap.map.is_some()); // map should still be allocated
}

