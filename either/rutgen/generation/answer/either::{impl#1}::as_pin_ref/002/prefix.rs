// Answer 0

#[test]
fn test_as_pin_ref_left() {
    use std::pin::Pin;
    use std::marker::PhantomPinned;

    struct PinnedValue {
        _pin: PhantomPinned,
        value: i32,
    }

    let value = PinnedValue {
        _pin: PhantomPinned,
        value: 10,
    };

    let either = Either::Left(value);
    let pinned_either = Pin::new(&either);

    let result: Either<Pin<&PinnedValue>, ()> = pinned_either.as_pin_ref();
}

#[test]
fn test_as_pin_ref_left_with_string() {
    use std::pin::Pin;
    use std::marker::PhantomPinned;

    struct PinnedString {
        _pin: PhantomPinned,
        value: String,
    }

    let value = PinnedString {
        _pin: PhantomPinned,
        value: String::from("Test"),
    };

    let either = Either::Left(value);
    let pinned_either = Pin::new(&either);

    let result: Either<Pin<&PinnedString>, ()> = pinned_either.as_pin_ref();
}

#[test]
fn test_as_pin_ref_left_with_custom_struct() {
    use std::pin::Pin;
    use std::marker::PhantomPinned;

    struct CustomStruct {
        _pin: PhantomPinned,
        data: f64,
    }

    let value = CustomStruct {
        _pin: PhantomPinned,
        data: 3.14,
    };

    let either = Either::Left(value);
    let pinned_either = Pin::new(&either);

    let result: Either<Pin<&CustomStruct>, ()> = pinned_either.as_pin_ref();
}

