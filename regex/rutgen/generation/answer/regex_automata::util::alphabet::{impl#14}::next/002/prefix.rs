// Answer 0

#[test]
fn test_next_boundary_case() {
    struct MockBitSet {
        bits: [bool; 256],
    }

    impl MockBitSet {
        fn new() -> Self {
            let mut bits = [false; 256];
            for i in 0..255 {
                bits[i] = true;
            }
            bits[255] = true; // contains 255
            MockBitSet { bits }
        }

        fn contains(&self, byte: u8) -> bool {
            self.bits[usize::from(byte)]
        }
    }

    let bitset = MockBitSet::new();
    let mut byte_set = ByteSet { bits: bitset };
    let mut iter = ByteSetRangeIter { set: &byte_set, b: 255 };

    let result = iter.next(); // expects Some((254, 255))

    // function call only, no assert
    let _ = result;
}

#[test]
fn test_next_with_non_contiguous_range() {
    struct MockBitSet {
        bits: [bool; 256],
    }

    impl MockBitSet {
        fn new() -> Self {
            let mut bits = [false; 256];
            for i in 0..254 {
                bits[i] = true; // contains 0-253
            }
            bits[255] = false; // does not contain 255
            MockBitSet { bits }
        }

        fn contains(&self, byte: u8) -> bool {
            self.bits[usize::from(byte)]
        }
    }

    let bitset = MockBitSet::new();
    let mut byte_set = ByteSet { bits: bitset };
    let mut iter = ByteSetRangeIter { set: &byte_set, b: 255 };

    let result = iter.next(); // expects Some((254, 255))

    // function call only, no assert
    let _ = result;
}

