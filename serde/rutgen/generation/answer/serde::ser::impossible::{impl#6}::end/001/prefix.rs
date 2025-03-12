// Answer 0

#[test]
fn test_end_empty_impossible() {
    struct OkType;
    struct MyError;

    let impossible_instance: Impossible<OkType, MyError> = Impossible {
        void: std::mem::zeroed(),
        ok: PhantomData,
        error: PhantomData,
    };
    let _result: Result<OkType, MyError> = impossible_instance.end();
}

#[test]
#[should_panic]
fn test_end_panic_impossible() {
    struct OkType;
    struct MyError;

    let impossible_instance: Impossible<OkType, MyError> = Impossible {
        void: std::mem::zeroed(),
        ok: PhantomData,
        error: PhantomData,
    };
    let _result: Result<OkType, MyError> = impossible_instance.end();
}

