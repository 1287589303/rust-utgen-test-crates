// Answer 0

#[test]
fn test_size_hint_zero_length() {
    struct TestCodeUnit;
    impl PunycodeCodeUnit for TestCodeUnit {
        fn is_delimiter(&self) -> bool { false }
        fn is_ascii(&self) -> bool { true }
        fn digit(&self) -> Option<u32> { None }
        fn char(&self) -> char { 'a' }
        fn char_ascii_lower_case(&self) -> char { 'a' }
    }

    struct TestCaller;
    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let base: Vec<TestCodeUnit> = Vec::new();
    let insertions: [(usize, char); 0] = [];
    let mut decode = Decode {
        base: base.iter(),
        insertions: &insertions,
        inserted: 0,
        position: 0,
        len: 0,
        phantom: PhantomData,
    };
    decode.size_hint();
}

#[test]
fn test_size_hint_full_length() {
    struct TestCodeUnit;
    impl PunycodeCodeUnit for TestCodeUnit {
        fn is_delimiter(&self) -> bool { false }
        fn is_ascii(&self) -> bool { true }
        fn digit(&self) -> Option<u32> { None }
        fn char(&self) -> char { 'a' }
        fn char_ascii_lower_case(&self) -> char { 'a' }
    }

    struct TestCaller;
    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let base: Vec<TestCodeUnit> = vec![TestCodeUnit; 5];
    let insertions: [(usize, char); 0] = [];
    let mut decode = Decode {
        base: base.iter(),
        insertions: &insertions,
        inserted: 0,
        position: 5,
        len: 5,
        phantom: PhantomData,
    };
    decode.size_hint();
}

#[test]
fn test_size_hint_partial_length() {
    struct TestCodeUnit;
    impl PunycodeCodeUnit for TestCodeUnit {
        fn is_delimiter(&self) -> bool { false }
        fn is_ascii(&self) -> bool { true }
        fn digit(&self) -> Option<u32> { None }
        fn char(&self) -> char { 'a' }
        fn char_ascii_lower_case(&self) -> char { 'a' }
    }

    struct TestCaller;
    impl PunycodeCaller for TestCaller {
        const EXTERNAL_CALLER: bool = false;
    }

    let base: Vec<TestCodeUnit> = vec![TestCodeUnit; 10];
    let insertions: [(usize, char); 0] = [];
    let mut decode = Decode {
        base: base.iter(),
        insertions: &insertions,
        inserted: 0,
        position: 4,
        len: 10,
        phantom: PhantomData,
    };
    decode.size_hint();
}

