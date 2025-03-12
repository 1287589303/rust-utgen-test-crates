// Answer 0

#[test]
fn test_lookup_with_collision_resolution() {
    struct MapEntry {
        key: u32,
        value: usize,
    }

    struct HashMap {
        map: Vec<MapEntry>,
        mask: u32,
    }

    impl HashMap {
        fn new() -> Self {
            let size = 8;
            HashMap {
                map: vec![
                    MapEntry { key: 1, value: 10 },
                    MapEntry { key: 2, value: 20 },
                    MapEntry { key: 3, value: 30 },
                    MapEntry { key: 4, value: 40 },
                    MapEntry { key: 5, value: 50 },
                    MapEntry { key: 6, value: 60 },
                    MapEntry { key: 7, value: 70 },
                    MapEntry { key: 0, value: Default::default() },
                ],
                mask: size as u32 - 1,
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

    let hashmap = HashMap::new();
    let result = hashmap.lookup(5);
    assert_eq!(result, 0); // Expecting the index to resolve to the empty/default entry
}

#[test]
fn test_lookup_with_perturbation() {
    struct MapEntry {
        key: u32,
        value: usize,
    }

    struct HashMap {
        map: Vec<MapEntry>,
        mask: u32,
    }

    impl HashMap {
        fn new() -> Self {
            let size = 8;
            HashMap {
                map: vec![
                    MapEntry { key: 1, value: 10 },
                    MapEntry { key: 5, value: 20 },
                    MapEntry { key: 2, value: 30 },
                    MapEntry { key: 3, value: 40 },
                    MapEntry { key: 4, value: 50 },
                    MapEntry { key: 6, value: 60 },
                    MapEntry { key: 7, value: 70 },
                    MapEntry { key: 0, value: Default::default() },
                ],
                mask: size as u32 - 1,
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

    let hashmap = HashMap::new();
    let result = hashmap.lookup(3);
    assert_eq!(result, 7); // Expecting the index to resolve to the empty/default entry after perturbation
}

