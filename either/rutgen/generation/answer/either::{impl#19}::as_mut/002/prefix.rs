// Answer 0

#[test]
fn test_either_as_mut_left_empty_slice() {
    struct SliceWrapper<'a>(&'a mut [i32]);
    impl AsMut<[i32]> for SliceWrapper<'_> {
        fn as_mut(&mut self) -> &mut [i32] {
            self.0
        }
    }

    let mut left: Either<SliceWrapper<'static>, ()> = Either::Left(SliceWrapper(&mut []));
    let _result: &mut [i32] = left.as_mut();
}

#[test]
fn test_either_as_mut_left_single_element_slice() {
    struct SliceWrapper<'a>(&'a mut [i32]);
    impl AsMut<[i32]> for SliceWrapper<'_> {
        fn as_mut(&mut self) -> &mut [i32] {
            self.0
        }
    }

    let mut left_data: [i32; 1] = [1];
    let mut left: Either<SliceWrapper<'static>, ()> = Either::Left(SliceWrapper(&mut left_data));
    let _result: &mut [i32] = left.as_mut();
}

#[test]
fn test_either_as_mut_left_max_capacity_slice() {
    struct SliceWrapper<'a>(&'a mut [i32]);
    impl AsMut<[i32]> for SliceWrapper<'_> {
        fn as_mut(&mut self) -> &mut [i32] {
            self.0
        }
    }

    let mut left_data: [i32; 1024] = [0; 1024];
    let mut left: Either<SliceWrapper<'static>, ()> = Either::Left(SliceWrapper(&mut left_data));
    let _result: &mut [i32] = left.as_mut();
}

#[test]
fn test_either_as_mut_right_empty_slice() {
    struct SliceWrapper<'a>(&'a mut [i32]);
    impl AsMut<[i32]> for SliceWrapper<'_> {
        fn as_mut(&mut self) -> &mut [i32] {
            self.0
        }
    }

    let mut right: Either<(), SliceWrapper<'static>> = Either::Right(SliceWrapper(&mut []));
    let _result: &mut [i32] = right.as_mut();
}

#[test]
fn test_either_as_mut_right_single_element_slice() {
    struct SliceWrapper<'a>(&'a mut [i32]);
    impl AsMut<[i32]> for SliceWrapper<'_> {
        fn as_mut(&mut self) -> &mut [i32] {
            self.0
        }
    }

    let mut right_data: [i32; 1] = [2];
    let mut right: Either<(), SliceWrapper<'static>> = Either::Right(SliceWrapper(&mut right_data));
    let _result: &mut [i32] = right.as_mut();
}

#[test]
fn test_either_as_mut_right_max_capacity_slice() {
    struct SliceWrapper<'a>(&'a mut [i32]);
    impl AsMut<[i32]> for SliceWrapper<'_> {
        fn as_mut(&mut self) -> &mut [i32] {
            self.0
        }
    }

    let mut right_data: [i32; 1024] = [0; 1024];
    let mut right: Either<(), SliceWrapper<'static>> = Either::Right(SliceWrapper(&mut right_data));
    let _result: &mut [i32] = right.as_mut();
}

