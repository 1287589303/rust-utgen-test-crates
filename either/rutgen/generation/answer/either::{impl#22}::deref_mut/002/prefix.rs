// Answer 0

#[test]
fn test_deref_mut_left() {
    struct Inner { value: i32 }
    impl DerefMut for Inner {
        fn deref_mut(&mut self) -> &mut Self {
            self
        }
    }

    let mut left_value = Inner { value: 10 };
    let mut either: Either<Inner, ()> = Either::Left(left_value);
    let result = either.deref_mut();
    result.value = 20; // Modify to check if deref_mut works
}

#[test]
fn test_deref_mut_right() {
    struct Inner { value: i32 }
    impl DerefMut for Inner {
        fn deref_mut(&mut self) -> &mut Self {
            self
        }
    }

    let mut right_value = Inner { value: 30 };
    let mut either: Either<(), Inner> = Either::Right(right_value);
    let result = either.deref_mut();
    result.value = 40; // Modify to check if deref_mut works
}

#[test]
fn test_deref_mut_boundary_left() {
    struct Inner { value: i32 }
    impl DerefMut for Inner {
        fn deref_mut(&mut self) -> &mut Self {
            self
        }
    }
    
    let mut left_value = Inner { value: i32::MIN };
    let mut either: Either<Inner, ()> = Either::Left(left_value);
    let result = either.deref_mut();
    result.value = i32::MAX; // Check boundary modification
}

#[test]
fn test_deref_mut_boundary_right() {
    struct Inner { value: i32 }
    impl DerefMut for Inner {
        fn deref_mut(&mut self) -> &mut Self {
            self
        }
    }

    let mut right_value = Inner { value: i32::MAX };
    let mut either: Either<(), Inner> = Either::Right(right_value);
    let result = either.deref_mut();
    result.value = i32::MIN; // Check boundary modification
}

