// Answer 0

#[test]
fn test_as_mut_left() {
    struct LeftVec(Vec<i32>);
    impl AsMut<[i32]> for LeftVec {
        fn as_mut(&mut self) -> &mut [i32] {
            self.0.as_mut()
        }
    }

    struct RightVec(Vec<i32>);
    impl AsMut<[i32]> for RightVec {
        fn as_mut(&mut self) -> &mut [i32] {
            self.0.as_mut()
        }
    }

    let mut left = LeftVec(vec![1, 2, 3]);
    let either = Either::Left(left);
    let slice: &mut [i32] = either.as_mut();
}

#[test]
fn test_as_mut_right() {
    struct LeftVec(Vec<i32>);
    impl AsMut<[i32]> for LeftVec {
        fn as_mut(&mut self) -> &mut [i32] {
            self.0.as_mut()
        }
    }

    struct RightVec(Vec<i32>);
    impl AsMut<[i32]> for RightVec {
        fn as_mut(&mut self) -> &mut [i32] {
            self.0.as_mut()
        }
    }

    let mut right = RightVec(vec![4, 5, 6]);
    let either = Either::Right(right);
    let slice: &mut [i32] = either.as_mut();
}

#[test]
fn test_as_mut_left_empty() {
    struct LeftVec(Vec<i32>);
    impl AsMut<[i32]> for LeftVec {
        fn as_mut(&mut self) -> &mut [i32] {
            self.0.as_mut()
        }
    }

    struct RightVec(Vec<i32>);
    impl AsMut<[i32]> for RightVec {
        fn as_mut(&mut self) -> &mut [i32] {
            self.0.as_mut()
        }
    }

    let mut left = LeftVec(vec![]);
    let either = Either::Left(left);
    let slice: &mut [i32] = either.as_mut();
}

#[test]
fn test_as_mut_right_empty() {
    struct LeftVec(Vec<i32>);
    impl AsMut<[i32]> for LeftVec {
        fn as_mut(&mut self) -> &mut [i32] {
            self.0.as_mut()
        }
    }

    struct RightVec(Vec<i32>);
    impl AsMut<[i32]> for RightVec {
        fn as_mut(&mut self) -> &mut [i32] {
            self.0.as_mut()
        }
    }

    let mut right = RightVec(vec![]);
    let either = Either::Right(right);
    let slice: &mut [i32] = either.as_mut();
}

