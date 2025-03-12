// Answer 0

#[test]
fn test_lookup_key_found_after_collision() {
    struct MapEntry {
        key: u32,
        value: usize,
    }

    struct TestStruct {
        mask: u32,
        map: Vec<MapEntry>,
    }

    impl TestStruct {
        fn new(mask: u32, entries: Vec<MapEntry>) -> Self {
            Self { mask, map: entries }
        }

        fn lookup(&self, key: u32) -> usize {
            let hash = key;
            let mut i = hash as usize & self.mask as usize;

            if self.map[i].value == Default::default() || self.map[i].key == key {
                return i;
            }

            let mut perturb = key;
            loop {
                i = (i * 5 + perturb as usize + 1) & self.mask as usize;

                if self.map[i].value == Default::default() || self.map[i].key == key {
                    return i;
                }

                perturb >>= 5;
            }
        }
    }

    // Set up the test
    let mask = 3;
    let entries = vec![
        MapEntry { key: 1, value: 10 },
        MapEntry { key: 2, value: 20 },
        MapEntry { key: 3, value: 30 },
        MapEntry { key: 4, value: 40 },
    ];
    let test_struct = TestStruct::new(mask, entries);
    let key_to_lookup = 4;

    // Execute the lookup
    let result = test_struct.lookup(key_to_lookup);

    // Asserting that the lookup returns the index corresponding to key = 4
    assert_eq!(result, 3);
}

#[test]
fn test_lookup_key_found_with_collision() {
    struct MapEntry {
        key: u32,
        value: usize,
    }

    struct TestStruct {
        mask: u32,
        map: Vec<MapEntry>,
    }

    impl TestStruct {
        fn new(mask: u32, entries: Vec<MapEntry>) -> Self {
            Self { mask, map: entries }
        }

        fn lookup(&self, key: u32) -> usize {
            let hash = key;
            let mut i = hash as usize & self.mask as usize;

            if self.map[i].value == Default::default() || self.map[i].key == key {
                return i;
            }

            let mut perturb = key;
            loop {
                i = (i * 5 + perturb as usize + 1) & self.mask as usize;

                if self.map[i].value == Default::default() || self.map[i].key == key {
                    return i;
                }

                perturb >>= 5;
            }
        }
    }

    // Set up the test
    let mask = 3;
    let entries = vec![
        MapEntry { key: 1, value: 10 },
        MapEntry { key: 2, value: 20 },
        MapEntry { key: 5, value: 30 }, // Collision will occur here for mask = 3
        MapEntry { key: 4, value: 40 },
    ];
    let test_struct = TestStruct::new(mask, entries);
    let key_to_lookup = 4;

    // Execute the lookup
    let result = test_struct.lookup(key_to_lookup);

    // Asserting that the lookup returns the index corresponding to key = 4
    assert_eq!(result, 3);
}

