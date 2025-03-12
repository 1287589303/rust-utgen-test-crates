// Answer 0

#[test]
fn test_grow_increases_capacity() {
    struct GrowingHashmap<ValueType> {
        map: Option<Vec<GrowingHashmapMapElemChar<ValueType>>>,
        mask: usize,
        used: usize,
        fill: usize,
    }

    struct GrowingHashmapMapElemChar<ValueType> {
        key: char,
        value: ValueType,
    }

    impl<ValueType: Default + Copy> GrowingHashmap<ValueType> {
        fn new() -> Self {
            GrowingHashmap {
                map: Some(vec![GrowingHashmapMapElemChar::default(); 1]),
                mask: 0,
                used: 0,
                fill: 0,
            }
        }

        fn lookup(&self, key: char) -> usize {
            // Simple hash function for demonstration purposes
            key as usize & self.mask
        }

        fn grow(&mut self, min_used: usize) {
            let mut new_size = self.mask + 1;
            while new_size <= min_used {
                new_size <<= 1;
            }

            self.fill = self.used;
            self.mask = new_size - 1;

            let old_map = std::mem::replace(
                self.map.as_mut().expect("callers have to ensure map is allocated"),
                vec![GrowingHashmapMapElemChar::<ValueType>::default(); new_size],
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

    let mut hashmap = GrowingHashmap::<i32>::new();
    hashmap.grow(2);
    assert!(hashmap.mask >= 1);
}

#[test]
fn test_grow_does_not_crash_on_empty() {
    struct GrowingHashmap<ValueType> {
        map: Option<Vec<GrowingHashmapMapElemChar<ValueType>>>,
        mask: usize,
        used: usize,
        fill: usize,
    }

    struct GrowingHashmapMapElemChar<ValueType> {
        key: char,
        value: ValueType,
    }

    impl<ValueType: Default + Copy> GrowingHashmap<ValueType> {
        fn new() -> Self {
            GrowingHashmap {
                map: Some(vec![GrowingHashmapMapElemChar::default(); 1]),
                mask: 0,
                used: 0,
                fill: 0,
            }
        }

        fn grow(&mut self, min_used: usize) {
            let mut new_size = self.mask + 1;
            while new_size <= min_used {
                new_size <<= 1;
            }

            self.fill = self.used;
            self.mask = new_size - 1;

            let old_map = std::mem::replace(
                self.map.as_mut().expect("callers have to ensure map is allocated"),
                vec![GrowingHashmapMapElemChar::<ValueType>::default(); new_size],
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

    let mut hashmap = GrowingHashmap::<i32>::new();
    hashmap.grow(0);
    assert_eq!(hashmap.used, 0);
    assert!(hashmap.map.is_some());
}

