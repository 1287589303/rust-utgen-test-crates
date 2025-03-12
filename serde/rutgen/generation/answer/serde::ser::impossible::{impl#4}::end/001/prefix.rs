// Answer 0

#[test]
fn test_end_valid_instance() {
    struct ValidOk;
    struct ValidError;

    let instance: Impossible<ValidOk, ValidError> = Impossible {
        void: std::mem::MaybeUninit::uninit().assume_init(),
        ok: PhantomData,
        error: PhantomData,
    };

    let _result = instance.end();
}

#[should_panic]
#[test]
fn test_end_invalid_instance() {
    struct PanicOk;
    struct PanicError;

    let instance: Impossible<PanicOk, PanicError> = Impossible {
        void: std::mem::MaybeUninit::uninit().assume_init(),
        ok: PhantomData,
        error: PhantomData,
    };

    let _result = instance.end();
}

