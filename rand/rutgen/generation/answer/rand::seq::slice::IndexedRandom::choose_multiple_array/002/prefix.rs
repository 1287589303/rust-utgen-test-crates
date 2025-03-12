// Answer 0

#[test]
fn test_choose_multiple_array_with_non_empty_slice() {
    struct TestArray<'a> {
        data: &'a [u8],
    }

    impl<'a> Index<usize> for TestArray<'a> {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<'a> IndexedRandom for TestArray<'a> {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let mut rng = &mut rand::rng();
    let sample = b"Hello, audience!";
    let array = TestArray { data: sample };

    let a: [u8; 3] = array.choose_multiple_array(&mut rng).unwrap();
}

#[test]
fn test_choose_multiple_array_with_exact_length() {
    struct TestArray<'a> {
        data: &'a [u8],
    }

    impl<'a> Index<usize> for TestArray<'a> {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<'a> IndexedRandom for TestArray<'a> {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let mut rng = &mut rand::rng();
    let sample = b"Hello!";
    let array = TestArray { data: sample };

    let a: [u8; 6] = array.choose_multiple_array(&mut rng).unwrap();
}

#[test]
fn test_choose_multiple_array_with_zero_length() {
    struct TestArray<'a> {
        data: &'a [u8],
    }

    impl<'a> Index<usize> for TestArray<'a> {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<'a> IndexedRandom for TestArray<'a> {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let mut rng = &mut rand::rng();
    let sample = b"Data";
    let array = TestArray { data: sample };

    let a: [u8; 0] = array.choose_multiple_array(&mut rng).unwrap();
}

#[test]
#[should_panic]
fn test_choose_multiple_array_exceeds_length() {
    struct TestArray<'a> {
        data: &'a [u8],
    }

    impl<'a> Index<usize> for TestArray<'a> {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<'a> IndexedRandom for TestArray<'a> {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let mut rng = &mut rand::rng();
    let sample = b"Hello, audience!";
    let array = TestArray { data: sample };

    // N is larger than the length of the slice
    let _a: [u8; 20] = array.choose_multiple_array(&mut rng).unwrap();
}

