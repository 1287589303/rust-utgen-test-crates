// Answer 0

#[test]
fn test_lookup_with_default_value() {
    struct MapEntry {
        key: u32,
        value: usize,
    }

    struct HashMap {
        mask: u32,
        map: Vec<MapEntry>,
    }

    impl HashMap {
        fn new(size: usize) -> Self {
            HashMap {
                mask: (size - 1) as u32,
                map: vec![MapEntry { key: 0, value: Default::default() }; size],
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

    let size = 8; // example size that is a power of two
    let hashmap = HashMap::new(size);
    let key = 42; // arbitrary key

    let result = hashmap.lookup(key);
    assert_eq!(result, (key as usize & hashmap.mask as usize));
}

#[test]
fn test_lookup_with_non_default_value() {
    struct MapEntry {
        key: u32,
        value: usize,
    }

    struct HashMap {
        mask: u32,
        map: Vec<MapEntry>,
    }

    impl HashMap {
        fn new(size: usize) -> Self {
            let mut map = vec![MapEntry { key: 0, value: Default::default() }; size];
            map[1] = MapEntry { key: 42, value: 1 }; // non-default entry
            HashMap {
                mask: (size - 1) as u32,
                map,
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

    let size = 8; // example size that is a power of two
    let hashmap = HashMap::new(size);
    let key = 42; // will resolve to index 1 due to a collision

    let result = hashmap.lookup(key);
    assert_eq!(result, 1);
}

