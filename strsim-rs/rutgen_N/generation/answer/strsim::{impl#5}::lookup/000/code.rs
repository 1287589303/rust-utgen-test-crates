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
        fn new(size: usize) -> Self {
            Self {
                map: vec![MapEntry { key: Default::default(), value: Default::default(); size }],
                mask: (size - 1) as u32,
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

    let mut hashmap = HashMap::new(16);
    hashmap.map[1] = MapEntry { key: 1, value: 10 };

    let index = hashmap.lookup(1);
    assert_eq!(index, 1);
}

#[test]
fn test_lookup_with_nonexistent_key() {
    struct MapEntry {
        key: u32,
        value: usize,
    }

    struct HashMap {
        map: Vec<MapEntry>,
        mask: u32,
    }

    impl HashMap {
        fn new(size: usize) -> Self {
            Self {
                map: vec![MapEntry { key: Default::default(), value: Default::default(); size }],
                mask: (size - 1) as u32,
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

    let hashmap = HashMap::new(16);

    let index = hashmap.lookup(2);
    assert_eq!(index, 2);
}

#[test]
fn test_lookup_edge_case() {
    struct MapEntry {
        key: u32,
        value: usize,
    }

    struct HashMap {
        map: Vec<MapEntry>,
        mask: u32,
    }

    impl HashMap {
        fn new(size: usize) -> Self {
            Self {
                map: vec![MapEntry { key: Default::default(), value: Default::default(); size }],
                mask: (size - 1) as u32,
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

    let mut hashmap = HashMap::new(32);
    hashmap.map[31] = MapEntry { key: 31, value: 20 };

    let index = hashmap.lookup(31);
    assert_eq!(index, 31);
}

