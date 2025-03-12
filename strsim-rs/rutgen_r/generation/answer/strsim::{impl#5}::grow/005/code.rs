// Answer 0

#[derive(Default)]
struct GrowingHashmapMapElemChar {
    key: char,
    value: i32,
}

struct GrowingHashmap {
    map: Option<Vec<GrowingHashmapMapElemChar>>,
    mask: usize,
    used: i32,
    fill: i32,
}

impl GrowingHashmap {
    fn new() -> Self {
        GrowingHashmap {
            map: Some(Vec::new()),
            mask: 0,
            used: 0,
            fill: 0,
        }
    }

    fn lookup(&self, key: char) -> usize {
        // Dummy implementation for test purposes
        key as usize % self.mask
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
fn test_grow_with_conditions() {
    let mut hashmap = GrowingHashmap::new();
    hashmap.mask = 3; // Initial mask
    hashmap.used = 3; // Set used to a value greater than 0
    hashmap.fill = 3; // Assuming we had 3 elements to start

    hashmap.map = Some(vec![
        GrowingHashmapMapElemChar { key: 'a', value: 1 },
        GrowingHashmapMapElemChar { key: 'b', value: 2 },
        GrowingHashmapMapElemChar { key: 'c', value: 3 },
    ]);

    hashmap.grow(3); // Calling grow with min_used <= mask

    assert_eq!(hashmap.mask, 7); // mask should be adjusted
    assert_eq!(hashmap.used, 3); // The used count should remain the same
    assert!(hashmap.map.is_some());
    assert_eq!(hashmap.map.as_ref().unwrap().len(), 8); // New size should be 8
    assert_eq!(hashmap.map.as_ref().unwrap()[0].value, 1); // First element should be 1
    assert_eq!(hashmap.map.as_ref().unwrap()[1].value, 2); // Second element should be 2
    assert_eq!(hashmap.map.as_ref().unwrap()[2].value, 3); // Third element should be 3
}

