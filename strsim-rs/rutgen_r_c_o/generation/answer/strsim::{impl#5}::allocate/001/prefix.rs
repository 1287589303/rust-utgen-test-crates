// Answer 0

#[test]
fn test_allocate_with_initial_values() {
    let mut hashmap: GrowingHashmapChar<u32> = GrowingHashmapChar {
        used: 0,
        fill: 0,
        mask: 0,
        map: None,
    };
    hashmap.allocate();
}

#[test]
fn test_allocate_with_nonzero_initial_used_and_fill() {
    let mut hashmap: GrowingHashmapChar<u32> = GrowingHashmapChar {
        used: 3,
        fill: 3,
        mask: 0,
        map: None,
    };
    hashmap.allocate();
}

#[test]
fn test_allocate_after_some_growth() {
    let mut hashmap: GrowingHashmapChar<u32> = GrowingHashmapChar {
        used: 7,
        fill: 7,
        mask: 0,
        map: None,
    };
    hashmap.allocate();
}

