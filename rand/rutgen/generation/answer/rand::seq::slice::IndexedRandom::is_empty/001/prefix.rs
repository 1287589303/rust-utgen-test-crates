// Answer 0

#[test]
fn test_is_empty_with_empty_array() {
    struct TestArray {
        data: [i32; 0],
    }

    impl Index<usize> for TestArray {
        type Output = i32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl IndexedRandom for TestArray {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let empty_array = TestArray { data: [] };
    let result = empty_array.is_empty();
}

#[test]
fn test_is_empty_with_non_empty_array() {
    struct TestArray {
        data: [i32; 3],
    }

    impl Index<usize> for TestArray {
        type Output = i32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl IndexedRandom for TestArray {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let non_empty_array = TestArray { data: [1, 2, 3] };
    let result = non_empty_array.is_empty();
}

#[test]
fn test_is_empty_with_larger_non_empty_array() {
    struct TestArray {
        data: [i32; 5],
    }

    impl Index<usize> for TestArray {
        type Output = i32;

        fn index(&self, index: usize) -> &Self::Output {
            &self.data[index]
        }
    }

    impl IndexedRandom for TestArray {
        fn len(&self) -> usize {
            self.data.len()
        }
    }

    let larger_non_empty_array = TestArray { data: [1, 2, 3, 4, 5] };
    let result = larger_non_empty_array.is_empty();
}

