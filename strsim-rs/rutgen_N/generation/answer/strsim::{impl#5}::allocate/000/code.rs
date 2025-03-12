// Answer 0

#[test]
fn test_allocate_initializes_mask_and_map() {
    struct GrowingHashmap {
        mask: usize,
        map: Option<Vec<GrowingHashmapMapElemChar>>,
    }

    #[derive(Default)]
    struct GrowingHashmapMapElemChar;

    impl GrowingHashmap {
        fn allocate(&mut self) {
            self.mask = 8 - 1;
            self.map = Some(vec![GrowingHashmapMapElemChar::default(); 8]);
        }
    }

    let mut hashmap = GrowingHashmap {
        mask: 0,
        map: None,
    };

    hashmap.allocate();

    assert_eq!(hashmap.mask, 7);
    assert!(hashmap.map.is_some());
    assert_eq!(hashmap.map.as_ref().unwrap().len(), 8);
}

