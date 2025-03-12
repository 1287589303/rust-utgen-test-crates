// Answer 0

#[test]
fn test_next_byte_less_than_256_class_is_byte_false() {
    struct MockClass;
    impl MockClass {
        fn is_byte(&self, _: u8) -> bool {
            false
        }
        fn is_eoi(&self) -> bool {
            false
        }
    }

    let classes = ByteClasses::empty();
    let class = Unit(UnitKind::U8(0));
    let mut elements = ByteClassElements {
        classes: &classes,
        class,
        byte: 0,
    };

    while elements.byte < 256 {
        let result = elements.next();
        // We do not assert, as per instructions
    }
}

#[test]
fn test_next_byte_equal_to_256_class_is_byte_false() {
    struct MockClass;
    impl MockClass {
        fn is_byte(&self, _: u8) -> bool {
            false
        }
        fn is_eoi(&self) -> bool {
            false
        }
    }

    let classes = ByteClasses::empty();
    let class = Unit(UnitKind::U8(0));
    let mut elements = ByteClassElements {
        classes: &classes,
        class,
        byte: 256,
    };

    let result = elements.next();
    // We do not assert, as per instructions
}

#[test]
fn test_next_byte_equal_to_257_class_is_byte_false() {
    struct MockClass;
    impl MockClass {
        fn is_byte(&self, _: u8) -> bool {
            false
        }
        fn is_eoi(&self) -> bool {
            false
        }
    }

    let classes = ByteClasses::empty();
    let class = Unit(UnitKind::U8(0));
    let mut elements = ByteClassElements {
        classes: &classes,
        class,
        byte: 257,
    };

    let result = elements.next();
    // We do not assert, as per instructions
}

