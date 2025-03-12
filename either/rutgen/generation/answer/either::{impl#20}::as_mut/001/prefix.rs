// Answer 0

#[test]
fn test_as_mut_right_case_with_integer() {
    struct InnerArray([i32; 4]);
    impl AsMut<[i32]> for InnerArray {
        fn as_mut(&mut self) -> &mut [i32] {
            &mut self.0
        }
    }
    
    let mut either = Either::Right(InnerArray([1, 2, 3, 4]));
    let slice: &mut [i32] = either.as_mut(); 
}

#[test]
fn test_as_mut_right_case_with_float() {
    struct InnerArray([f32; 3]);
    impl AsMut<[f32]> for InnerArray {
        fn as_mut(&mut self) -> &mut [f32] {
            &mut self.0
        }
    }
    
    let mut either = Either::Right(InnerArray([1.1, 2.2, 3.3]));
    let slice: &mut [f32] = either.as_mut(); 
}

#[test]
fn test_as_mut_right_case_with_string() {
    struct InnerString(String);
    impl AsMut<[u8]> for InnerString {
        fn as_mut(&mut self) -> &mut [u8] {
            self.0.as_mut_bytes()
        }
    }

    let mut either = Either::Right(InnerString(String::from("Hello")));
    let slice: &mut [u8] = either.as_mut(); 
}

#[test]
fn test_as_mut_right_case_with_empty() {
    struct InnerEmpty(Vec<i32>);
    impl AsMut<[i32]> for InnerEmpty {
        fn as_mut(&mut self) -> &mut [i32] {
            self.0.as_mut()
        }
    }
    
    let mut either = Either::Right(InnerEmpty(Vec::new()));
    let slice: &mut [i32] = either.as_mut(); 
}

