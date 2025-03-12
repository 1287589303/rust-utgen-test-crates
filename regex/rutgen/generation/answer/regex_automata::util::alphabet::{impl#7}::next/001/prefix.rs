// Answer 0

#[test]
fn test_next_valid_byte() {
    struct MockClass {
        valid_bytes: Vec<u8>,
    }

    impl MockClass {
        fn is_byte(&self, byte: u8) -> bool {
            self.valid_bytes.contains(&byte)
        }
    }

    let byte_classes = ByteClasses([0; 256]);
    let valid_bytes = vec![0, 1, 2, 3, 4]; // valid byte examples
    let class = MockClass { valid_bytes };
    
    let mut elements = ByteClassElements {
        classes: &byte_classes,
        class: Unit::u8(0),
        byte: 0,
    };

    for expected_byte in 0..5 {
        elements.byte = expected_byte;
        let result = elements.next();
        // Call next() to be executed, and it should return Some(Unit::u8(expected_byte))
    }
}

#[test]
fn test_next_after_valid_bytes() {
    struct MockClass {
        valid_bytes: Vec<u8>,
    }

    impl MockClass {
        fn is_byte(&self, byte: u8) -> bool {
            self.valid_bytes.contains(&byte)
        }
    }

    let byte_classes = ByteClasses([0; 256]);
    let valid_bytes = vec![5, 6, 7]; // valid byte examples
    let class = MockClass { valid_bytes };
    
    let mut elements = ByteClassElements {
        classes: &byte_classes,
        class: Unit::u8(0),
        byte: 5,
    };

    for expected_byte in 5..8 {
        elements.byte = expected_byte;
        let result = elements.next();
        // Call next() to be executed, and it should return Some(Unit::u8(expected_byte))
    }
}

