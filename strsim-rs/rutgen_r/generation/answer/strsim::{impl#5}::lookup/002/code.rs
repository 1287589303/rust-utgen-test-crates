// Answer 0

#[test]
fn test_lookup_with_existing_key() {
    struct MapEntry {
        key: u32,
        value: usize,
    }

    struct HashMap {
        map: Vec<MapEntry>,
        mask: u32,
    }

    impl HashMap {
        fn new(size: usize, mask: u32) -> Self {
            Self {
                map: vec![MapEntry { key: 0, value: Default::default() }; size],
                mask,
            }
        }

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

    let mut hash_map = HashMap::new(10, 9); // Assuming a mask of 9 for a size of 10
    hash_map.map[2] = MapEntry { key: 42, value: 1 }; // Precondition: key 42 present in the map

    let result = hash_map.lookup(42); // Test lookup with existing key

    assert_eq!(result, 2); // i should be 2
}

#[test]
fn test_lookup_with_collided_key() {
    struct MapEntry {
        key: u32,
        value: usize,
    }

    struct HashMap {
        map: Vec<MapEntry>,
        mask: u32,
    }

    impl HashMap {
        fn new(size: usize, mask: u32) -> Self {
            Self {
                map: vec![MapEntry { key: 0, value: Default::default() }; size],
                mask,
            }
        }

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

    let mut hash_map = HashMap::new(10, 9); // Assuming a mask of 9 for a size of 10
    hash_map.map[2] = MapEntry { key: 42, value: 1 }; // Initial entry
    hash_map.map[3] = MapEntry { key: 99, value: 2 }; // Another occupied entry

    let result = hash_map.lookup(42); // Test lookup with key that collides

    assert_eq!(result, 2); // should return the original index for key 42
}

