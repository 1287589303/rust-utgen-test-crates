// Answer 0

#[test]
fn test_choose_multiple_weighted_non_zero_weights() {
    struct TestSlice<'a>(&'a [(char, f64)]);

    impl<'a> IndexedRandom for TestSlice<'a> {
        type Output = (char, f64);
        
        fn len(&self) -> usize {
            self.0.len()
        }

        #[inline]
        fn is_empty(&self) -> bool {
            self.len() == 0
        }
        
        fn choose<R>(&self, _: &mut R) -> Option<&Self::Output>
        where R: Rng + ?Sized {
            unimplemented!()
        }
    }

    let choices = TestSlice(&[('a', 2.0), ('b', 1.0), ('c', 1.0)]);
    let mut rng = rand::thread_rng();
    let _ = choices.choose_multiple_weighted(&mut rng, 2, |item| item.1);
}

#[test]
fn test_choose_multiple_weighted_equal_weights() {
    struct TestSlice<'a>(&'a [(char, f64)]);

    impl<'a> IndexedRandom for TestSlice<'a> {
        type Output = (char, f64);
        
        fn len(&self) -> usize {
            self.0.len()
        }

        #[inline]
        fn is_empty(&self) -> bool {
            self.len() == 0
        }

        fn choose<R>(&self, _: &mut R) -> Option<&Self::Output>
        where R: Rng + ?Sized {
            unimplemented!()
        }
    }

    let choices = TestSlice(&[('x', 1.0), ('y', 1.0), ('z', 1.0)]);
    let mut rng = rand::thread_rng();
    let _ = choices.choose_multiple_weighted(&mut rng, 2, |item| item.1);
}

#[test]
fn test_choose_multiple_weighted_single_item() {
    struct TestSlice<'a>(&'a [(char, f64)]);

    impl<'a> IndexedRandom for TestSlice<'a> {
        type Output = (char, f64);
        
        fn len(&self) -> usize {
            self.0.len()
        }

        #[inline]
        fn is_empty(&self) -> bool {
            self.len() == 0
        }

        fn choose<R>(&self, _: &mut R) -> Option<&Self::Output>
        where R: Rng + ?Sized {
            unimplemented!()
        }
    }

    let choices = TestSlice(&[('k', 1.0)]);
    let mut rng = rand::thread_rng();
    let _ = choices.choose_multiple_weighted(&mut rng, 1, |item| item.1);
}

#[test]
fn test_choose_multiple_weighted_boundary_case() {
    struct TestSlice<'a>(&'a [(char, f64)]);

    impl<'a> IndexedRandom for TestSlice<'a> {
        type Output = (char, f64);
        
        fn len(&self) -> usize {
            self.0.len()
        }

        #[inline]
        fn is_empty(&self) -> bool {
            self.len() == 0
        }

        fn choose<R>(&self, _: &mut R) -> Option<&Self::Output>
        where R: Rng + ?Sized {
            unimplemented!()
        }
    }

    let choices = TestSlice(&[('p', 3.0), ('q', 2.0), ('r', 1.0)]);
    let mut rng = rand::thread_rng();
    let _ = choices.choose_multiple_weighted(&mut rng, 3, |item| item.1);
}

