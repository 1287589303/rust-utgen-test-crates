// Answer 0

#[test]
fn test_next_with_char_valid_ascii_lowercase() {
    struct TestCodeUnit {
        value: char,
    }

    impl PunycodeCodeUnit for TestCodeUnit {
        fn is_delimiter(&self) -> bool { false }
        fn is_ascii(&self) -> bool { self.value.is_ascii() }
        fn digit(&self) -> Option<u32> { None }
        fn char(&self) -> char { self.value }
        fn char_ascii_lower_case(&self) -> char { self.value.to_ascii_lowercase() }
    }

    struct TestCaller;
    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let code_units = [TestCodeUnit { value: 'a' }, TestCodeUnit { value: 'b' }];
    let insertions = [(1, 'c')];
    let mut decode = Decode {
        base: code_units.iter(),
        insertions: &insertions,
        inserted: 0,
        position: 0,
        len: 2,
        phantom: PhantomData,
    };

    let result = decode.next();
}

#[test]
fn test_next_with_char_valid_non_ascii_lowercase() {
    struct TestCodeUnit {
        value: char,
    }

    impl PunycodeCodeUnit for TestCodeUnit {
        fn is_delimiter(&self) -> bool { false }
        fn is_ascii(&self) -> bool { self.value.is_ascii() }
        fn digit(&self) -> Option<u32> { None }
        fn char(&self) -> char { self.value }
        fn char_ascii_lower_case(&self) -> char { self.value.to_ascii_lowercase() }
    }

    struct TestCaller;
    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let code_units = [TestCodeUnit { value: '\u{00E9}' }, TestCodeUnit { value: '\u{00E0}' }];
    let insertions = [(1, 'c')];
    let mut decode = Decode {
        base: code_units.iter(),
        insertions: &insertions,
        inserted: 0,
        position: 0,
        len: 2,
        phantom: PhantomData,
    };

    let result = decode.next();
}

#[test]
fn test_next_with_multiple_insertions() {
    struct TestCodeUnit {
        value: char,
    }

    impl PunycodeCodeUnit for TestCodeUnit {
        fn is_delimiter(&self) -> bool { false }
        fn is_ascii(&self) -> bool { self.value.is_ascii() }
        fn digit(&self) -> Option<u32> { None }
        fn char(&self) -> char { self.value }
        fn char_ascii_lower_case(&self) -> char { self.value.to_ascii_lowercase() }
    }

    struct TestCaller;
    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let code_units = [TestCodeUnit { value: 'x' }, TestCodeUnit { value: 'y' }, TestCodeUnit { value: 'z' }];
    let insertions = [(0, 'a'), (2, 'b')];
    let mut decode = Decode {
        base: code_units.iter(),
        insertions: &insertions,
        inserted: 0,
        position: 0,
        len: 3,
        phantom: PhantomData,
    };

    let result = decode.next();
}

