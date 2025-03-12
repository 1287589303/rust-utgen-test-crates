// Answer 0

#[test]
fn test_choose_multiple_non_empty() {
    struct TestSlice<'a> {
        data: &'a [u8],
    }

    impl<'a> Index<usize> for TestSlice<'a> {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<'a> IndexedRandom for TestSlice<'a> {
        fn len(&self) -> usize {
            self.data.len()
        }
        
        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let mut rng = &mut rand::rng();
    let sample = TestSlice { data: b"Hello" };
    let amount = 3;
    let result = sample.choose_multiple(&mut rng, amount);
}

#[test]
fn test_choose_multiple_exact_amount() {
    struct TestSlice<'a> {
        data: &'a [u8],
    }

    impl<'a> Index<usize> for TestSlice<'a> {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<'a> IndexedRandom for TestSlice<'a> {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let mut rng = &mut rand::rng();
    let sample = TestSlice { data: b"World" };
    let amount = 5; // same as length
    let result = sample.choose_multiple(&mut rng, amount);
}

#[test]
fn test_choose_multiple_lower_bound_amount() {
    struct TestSlice<'a> {
        data: &'a [u8],
    }

    impl<'a> Index<usize> for TestSlice<'a> {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<'a> IndexedRandom for TestSlice<'a> {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let mut rng = &mut rand::rng();
    let sample = TestSlice { data: b"Random" };
    let amount = 1; // minimum non-zero amount
    let result = sample.choose_multiple(&mut rng, amount);
}

#[test]
fn test_choose_multiple_upper_bound_amount() {
    struct TestSlice<'a> {
        data: &'a [u8],
    }

    impl<'a> Index<usize> for TestSlice<'a> {
        type Output = u8;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl<'a> IndexedRandom for TestSlice<'a> {
        fn len(&self) -> usize {
            self.data.len()
        }

        fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    let mut rng = &mut rand::rng();
    let sample = TestSlice { data: b"Boundary" };
    let amount = 8; // maximum amount (same as length)
    let result = sample.choose_multiple(&mut rng, amount);
}

