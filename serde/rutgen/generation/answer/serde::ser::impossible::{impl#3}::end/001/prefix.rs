// Answer 0

#[test]
fn test_end_with_valid_ok_type() {
    struct ValidOk;
    struct ValidError;
    
    impl ser::Error for ValidError {}

    let impossible: Impossible<ValidOk, ValidError> = Impossible {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };

    impossible.end().unwrap();
}

#[should_panic]
#[test]
fn test_end_with_void_value() {
    struct ValidOk;
    struct ValidError;
    
    impl ser::Error for ValidError {}

    let impossible: Impossible<ValidOk, ValidError> = Impossible {
        void: Void {},
        ok: PhantomData,
        error: PhantomData,
    };

    impossible.end(); 
}

