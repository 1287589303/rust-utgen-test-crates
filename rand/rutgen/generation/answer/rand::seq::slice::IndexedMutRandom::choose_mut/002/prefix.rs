// Answer 0

#[test]
fn test_choose_mut_non_empty() {
    struct TestSlice<'a> {
        data: &'a mut [i32],
    }

    impl<'a> IndexedRandom for TestSlice<'a> {
        type Output = i32;

        fn len(&self) -> usize {
            self.data.len()
        }

        fn choose<R>(&self, rng: &mut R) -> Option<&Self::Output>
        where
            R: Rng + ?Sized,
        {
            if self.is_empty() { None } else { Some(&self.data[rng.random_range(..self.len())]) }
        }
    }

    impl<'a> Index<usize> for TestSlice<'a> {
        type Output = i32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    let mut rng = //... initialize your RNG here;
    let mut data = [1, 2, 3, 4, 5];
    let mut slice = TestSlice { data: &mut data };

    let result = slice.choose_mut(&mut rng);
}

#[test]
fn test_choose_mut_with_rng() {
    struct TestSlice<'a> {
        data: &'a mut [i32],
    }

    impl<'a> IndexedRandom for TestSlice<'a> {
        type Output = i32;

        fn len(&self) -> usize {
            self.data.len()
        }

        fn choose<R>(&self, rng: &mut R) -> Option<&Self::Output>
        where
            R: Rng + ?Sized,
        {
            if self.is_empty() { None } else { Some(&self.data[rng.random_range(..self.len())]) }
        }
    }

    impl<'a> Index<usize> for TestSlice<'a> {
        type Output = i32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    let mut rng = //... initialize your RNG here;
    let mut data = [10, 20, 30, 40, 50];
    let mut slice = TestSlice { data: &mut data };

    let result = slice.choose_mut(&mut rng);
} 

#[test]
fn test_choose_mut_multiple_elements() {
    struct TestSlice<'a> {
        data: &'a mut [i32],
    }

    impl<'a> IndexedRandom for TestSlice<'a> {
        type Output = i32;

        fn len(&self) -> usize {
            self.data.len()
        }

        fn choose<R>(&self, rng: &mut R) -> Option<&Self::Output>
        where
            R: Rng + ?Sized,
        {
            if self.is_empty() { None } else { Some(&self.data[rng.random_range(..self.len())]) }
        }
    }

    impl<'a> Index<usize> for TestSlice<'a> {
        type Output = i32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    let mut rng = //... initialize your RNG here;
    let mut data = [100, 200, 300, 400, 500];
    let mut slice = TestSlice { data: &mut data };

    let result = slice.choose_mut(&mut rng);
}

