// Answer 0

#[test]
#[should_panic]
fn test_end_with_void() {
    struct OkType;
    struct ErrorType;

    let impossible: Impossible<OkType, ErrorType> = Impossible {
        void: std::mem::transmute::<_, Void>(()),
        ok: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };
    impossible.end().unwrap();
}

#[test]
#[should_panic]
fn test_end_with_empty_ok() {
    struct EmptyOk;
    struct ErrorType;

    let impossible: Impossible<EmptyOk, ErrorType> = Impossible {
        void: std::mem::transmute::<_, Void>(()),
        ok: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };
    impossible.end().unwrap();
}

#[test]
#[should_panic]
fn test_end_with_empty_error() {
    struct OkType;
    struct EmptyError;

    let impossible: Impossible<OkType, EmptyError> = Impossible {
        void: std::mem::transmute::<_, Void>(()),
        ok: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };
    impossible.end().unwrap();
}

#[test]
#[should_panic]
fn test_end_with_non_empty_types() {
    struct NonEmptyOk;
    struct NonEmptyError;

    let impossible: Impossible<NonEmptyOk, NonEmptyError> = Impossible {
        void: std::mem::transmute::<_, Void>(()),
        ok: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };
    impossible.end().unwrap();
}

#[test]
#[should_panic]
fn test_end_with_invalid_error_type() {
    struct OkType;
    struct InvalidError;

    let impossible: Impossible<OkType, InvalidError> = Impossible {
        void: std::mem::transmute::<_, Void>(()),
        ok: std::marker::PhantomData,
        error: std::marker::PhantomData,
    };
    impossible.end().unwrap();
}

