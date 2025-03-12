// Answer 0

#[test]
fn test_choose_empty_slice() {
    struct EmptySlice;

    impl Index<usize> for EmptySlice {
        type Output = bool;

        fn index(&self, _: usize) -> &Self::Output {
            &false
        }
    }

    impl IndexedRandom for EmptySlice {
        fn len(&self) -> usize {
            0
        }
    }

    let empty_slice = EmptySlice;
    let mut rng = rand::rng();
    let result = empty_slice.choose(&mut rng);
}

#[test]
fn test_choose_empty_array() {
    struct EmptyArray;

    impl Index<usize> for EmptyArray {
        type Output = i32;

        fn index(&self, _: usize) -> &Self::Output {
            &0
        }
    }

    impl IndexedRandom for EmptyArray {
        fn len(&self) -> usize {
            0
        }
    }

    let empty_array = EmptyArray;
    let mut rng = rand::rng();
    let result = empty_array.choose(&mut rng);
}

