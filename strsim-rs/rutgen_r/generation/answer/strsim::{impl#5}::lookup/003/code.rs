// Answer 0

#[test]
fn test_lookup_non_default_key_non_default_value() {
    struct MapEntry {
        key: u32,
        value: usize,
    }

    struct HashMap {
        mask: u32,
        map: Vec<MapEntry>,
    }

    impl HashMap {
        fn lookup(&self, key: u32) -> usize {
            let hash = key;
            let mut i = hash as usize & self.mask as usize;

            let map = &self.map;

            if map[i].value == Default::default() || map[i].key == key {
                return i;
            }

            let mut perturb = key;
            loop {
                i = (i * 5 + perturb as usize + 1) & self.mask as usize;

                if map[i].value == Default::default() || map[i].key == key {
                    return i;
                }

                perturb >>= 5;
            }
        }
    }

    let map = vec![
        MapEntry { key: 1, value: 10 },
        MapEntry { key: 2, value: 20 },
        MapEntry { key: 3, value: 30 },
        MapEntry { key: 4, value: 40 },
        MapEntry { key: 5, value: 50 },
        MapEntry { key: 6, value: 60 },
        MapEntry { key: 7, value: 70 },
        MapEntry { key: 8, value: 80 },
        MapEntry { key: 9, value: 90 },
        MapEntry { key: 0, value: 0 }, // Default value entry
    ];

    let hashmap = HashMap {
        mask: 9, // Ensure the size is suitable for the entries
        map,
    };

    let result = hashmap.lookup(100); // A key that does not exist in the map
    assert_eq!(result, 0); // Should return the index of the default entry
}

#[test]
fn test_lookup_for_non_existent_key() {
    struct MapEntry {
        key: u32,
        value: usize,
    }

    struct HashMap {
        mask: u32,
        map: Vec<MapEntry>,
    }

    impl HashMap {
        fn lookup(&self, key: u32) -> usize {
            let hash = key;
            let mut i = hash as usize & self.mask as usize;

            let map = &self.map;

            if map[i].value == Default::default() || map[i].key == key {
                return i;
            }

            let mut perturb = key;
            loop {
                i = (i * 5 + perturb as usize + 1) & self.mask as usize;

                if map[i].value == Default::default() || map[i].key == key {
                    return i;
                }

                perturb >>= 5;
            }
        }
    }

    let map = vec![
        MapEntry { key: 10, value: 100 },
        MapEntry { key: 2, value: 200 },
        MapEntry { key: 3, value: 300 },
        MapEntry { key: 4, value: 400 },
        MapEntry { key: 5, value: 500 },
        MapEntry { key: 6, value: 600 },
        MapEntry { key: 7, value: 700 },
        MapEntry { key: 8, value: 800 },
        MapEntry { key: 9, value: 900 },
        MapEntry { key: 0, value: 0 }, // Default value entry
    ];

    let hashmap = HashMap {
        mask: 9, // Ensure the size is suitable for the entries
        map,
    };

    let result = hashmap.lookup(11); // A key that does not exist in the map
    assert_eq!(result, 0); // Should return the index of the default entry
}

