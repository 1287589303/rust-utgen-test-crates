// Answer 0

#[derive(Default)]
struct GrowingHashmapMapElemChar {
    key: char,
    value: i32,
}

struct GrowingHashmap {
    used: i32,
    fill: i32,
    mask: i32,
    map: Option<Vec<GrowingHashmapMapElemChar>>,
}

impl GrowingHashmap {
    fn new(size: i32) -> Self {
        GrowingHashmap {
            used: 0,
            fill: 0,
            mask: size - 1,
            map: Some(vec![GrowingHashmapMapElemChar::default(); size as usize]),
        }
    }

    fn lookup(&self, key: char) -> usize {
        key as usize % (self.mask + 1) as usize
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

#[test]
fn test_grow_new_size_greater_than_min_used() {
    let mut hashmap = GrowingHashmap::new(4);
    hashmap.used = 1;  // Set to a value greater than 0
    hashmap.fill = 1;  // Fill can be non-zero, but must not exceed used

    hashmap.map.as_mut().unwrap()[0] = GrowingHashmapMapElemChar { key: 'a', value: 1 }; // elem.value != Default::default() is true
    hashmap.map.as_mut().unwrap()[1] = GrowingHashmapMapElemChar::default(); // This will satify elem in old_map is false

    // Trigger grow with a min_used that will not be less than current size
    hashmap.grow(4);

    // After growth operation
    assert_eq!(hashmap.mask, 7); // For example, new_size should be 8 (11 - 1)
    assert_eq!(hashmap.used, 1);  // ensure that used does not end up zero
}

#[test]
fn test_grow_with_empty_old_map() {
    let mut hashmap = GrowingHashmap::new(4);
    hashmap.used = 0; // Should satisfy self.used == 0 is true
    hashmap.fill = 0;

    hashmap.grow(4);

    // Ensure that grow behaves correctly
    assert_eq!(hashmap.mask, 7); // New size should be 8
    assert_eq!(hashmap.used, 0); // It remains 0 as nothing was filled
}

#[test]
fn test_grow_with_multiple_elements() {
    let mut hashmap = GrowingHashmap::new(4);
    hashmap.used = 3;
    hashmap.fill = 3;

    hashmap.map.as_mut().unwrap()[0] = GrowingHashmapMapElemChar { key: 'a', value: 1 }; // Satisfies elem.value != Default::default()
    hashmap.map.as_mut().unwrap()[1] = GrowingHashmapMapElemChar { key: 'b', value: 2 };
    hashmap.map.as_mut().unwrap()[2] = GrowingHashmapMapElemChar::default(); // satisfy elem in old_map is false

    hashmap.grow(4);

    assert_eq!(hashmap.mask, 7); // Verify that the mask was updated
    assert_eq!(hashmap.used, 2);  // Verify used is decremented accordingly
}

