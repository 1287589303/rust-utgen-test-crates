// Answer 0

#[test]
fn test_as_pin_mut_left_variant() {
    use std::pin::Pin;

    struct InnerLeft {
        value: i32,
    }

    struct InnerRight {
        value: i32,
    }

    let mut either = Either::Left(InnerLeft { value: 42 });
    let mut pinned_either = Pin::new(&mut either);
    let result: Either<Pin<&mut InnerLeft>, Pin<&mut InnerRight>> = unsafe {
        pinned_either.as_pin_mut()
    };
}

#[test]
fn test_as_pin_mut_right_variant() {
    use std::pin::Pin;

    struct InnerLeft {
        value: i32,
    }

    struct InnerRight {
        value: i32,
    }

    let mut either = Either::Right(InnerRight { value: 42 });
    let mut pinned_either = Pin::new(&mut either);
    let result: Either<Pin<&mut InnerLeft>, Pin<&mut InnerRight>> = unsafe {
        pinned_either.as_pin_mut()
    };
}

