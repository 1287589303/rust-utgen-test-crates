// Answer 0

#[test]
fn test_hybrid_growing_hashmap_char_default() {
    struct HybridGrowingHashmapChar {
        map: GrowingHashmapChar,
        extended_ascii: [u8; 256],
    }
    
    struct GrowingHashmapChar;

    impl Default for GrowingHashmapChar {
        fn default() -> Self {
            GrowingHashmapChar
        }
    }

    impl Default for HybridGrowingHashmapChar {
        fn default() -> Self {
            HybridGrowingHashmapChar {
                map: GrowingHashmapChar::default(),
                extended_ascii: [Default::default(); 256],
            }
        }
    }

    let hashmap = HybridGrowingHashmapChar::default();
    assert_eq!(hashmap.extended_ascii.len(), 256);
}

