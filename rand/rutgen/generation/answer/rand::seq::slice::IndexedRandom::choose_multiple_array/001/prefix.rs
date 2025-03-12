// Answer 0

#[test]
fn test_choose_multiple_array_empty_slice() {
    struct TestSlice {
        data: Vec<u8>,
    }

    impl Index<usize> for TestSlice {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl IndexedRandom for TestSlice {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
    }

    let mut rng = &mut rand::rng();
    let sample = TestSlice { data: vec![] };
    let result: Option<[u8; 3]> = sample.choose_multiple_array(&mut rng);
}

#[test]
fn test_choose_multiple_array_insufficient_length() {
    struct TestSlice {
        data: Vec<u8>,
    }

    impl Index<usize> for TestSlice {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl IndexedRandom for TestSlice {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
    }

    let mut rng = &mut rand::rng();
    let sample = TestSlice { data: vec![1, 2] };
    let result: Option<[u8; 3]> = sample.choose_multiple_array(&mut rng);
}

#[test]
fn test_choose_multiple_array_exact_length() {
    struct TestSlice {
        data: Vec<u8>,
    }

    impl Index<usize> for TestSlice {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl IndexedRandom for TestSlice {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.data.is_empty()
        }
    }

    let mut rng = &mut rand::rng();
    let sample = TestSlice { data: vec![1, 2, 3] };
    let result: Option<[u8; 3]> = sample.choose_multiple_array(&mut rng);
}

