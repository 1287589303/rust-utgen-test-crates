// Answer 0

#[test]
fn test_byte_set_range_iter_next_case_1() {
    struct BitSet([bool; 256]);
    impl BitSet {
        fn contains(&self, byte: u8) -> bool {
            let bucket = byte / 128;
            let bit = byte % 128;
            self.0[usize::from(bucket)] & (1 << bit) > 0
        }
    }

    let mut bit_set = BitSet([false; 256]);
    bit_set.0[1] = 0b11111111; // Set all bits for range 128 to 255

    let byte_set = ByteSet {
        bits: bit_set,
    };

    let mut iter = ByteSetRangeIter {
        set: &byte_set,
        b: 255,
    };

    let result = iter.next();
}

#[test]
fn test_byte_set_range_iter_next_case_2() {
    struct BitSet([bool; 256]);
    impl BitSet {
        fn contains(&self, byte: u8) -> bool {
            let bucket = byte / 128;
            let bit = byte % 128;
            self.0[usize::from(bucket)] & (1 << bit) > 0
        }
    }

    let mut bit_set = BitSet([false; 256]);
    bit_set.0[1] = 0b11111111; // Set all bits for range 128 to 255

    let byte_set = ByteSet {
        bits: bit_set,
    };

    let mut iter = ByteSetRangeIter {
        set: &byte_set,
        b: 255,
    };

    // Call next and unpack result
    let result = iter.next();
}

