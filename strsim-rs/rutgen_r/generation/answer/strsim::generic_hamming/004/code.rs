// Answer 0

#[test]
fn test_generic_hamming_different_length_error() {
    struct VecWrapper(Vec<i32>);
    
    impl IntoIterator for VecWrapper {
        type Item = i32;
        type IntoIter = std::vec::IntoIter<i32>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }
    
    let a = VecWrapper(vec![1, 2, 3]);
    let b = VecWrapper(vec![1, 2]);
    
    let result = generic_hamming(a, b);
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
}

#[test]
fn test_generic_hamming_different_length_error_empty() {
    struct VecWrapper(Vec<i32>);
    
    impl IntoIterator for VecWrapper {
        type Item = i32;
        type IntoIter = std::vec::IntoIter<i32>;

        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }
    
    let a = VecWrapper(vec![1]);
    let b = VecWrapper(vec![]);
    
    let result = generic_hamming(a, b);
    assert_eq!(result, Err(StrSimError::DifferentLengthArgs));
}

