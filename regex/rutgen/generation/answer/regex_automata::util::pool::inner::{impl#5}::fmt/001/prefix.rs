// Answer 0

#[test]
fn test_fmt_with_non_empty_stack() {
    struct TestStruct;
    impl core::fmt::Debug for TestStruct {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("TestStruct")
        }
    }

    let stack = Mutex {
        locked: AtomicBool::new(false),
        data: UnsafeCell::new(vec![Box::new(TestStruct)]),
    };

    let create_fn = |_: &mut ()| Box::new(TestStruct);
    let pool: Pool<TestStruct, _> = Pool { stack, create: create_fn };

    let _ = pool.fmt(&mut core::fmt::Formatter::default());
}

#[test]
fn test_fmt_with_stack_of_multiple_elements() {
    struct AnotherTestStruct;
    impl core::fmt::Debug for AnotherTestStruct {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("AnotherTestStruct")
        }
    }

    let stack = Mutex {
        locked: AtomicBool::new(false),
        data: UnsafeCell::new(vec![Box::new(AnotherTestStruct), Box::new(AnotherTestStruct)]),
    };

    let create_fn = |_: &mut ()| Box::new(AnotherTestStruct);
    let pool: Pool<AnotherTestStruct, _> = Pool { stack, create: create_fn };

    let _ = pool.fmt(&mut core::fmt::Formatter::default());
}

#[test]
fn test_fmt_with_non_empty_stack_and_valid_owner_val() {
    struct ValidOwnerStruct;
    impl core::fmt::Debug for ValidOwnerStruct {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("ValidOwnerStruct")
        }
    }

    let stack = Mutex {
        locked: AtomicBool::new(false),
        data: UnsafeCell::new(vec![Box::new(ValidOwnerStruct)]),
    };

    let create_fn = |_: &mut ()| Box::new(ValidOwnerStruct);
    let pool: Pool<ValidOwnerStruct, _> = Pool { stack, create: create_fn };

    let _ = pool.fmt(&mut core::fmt::Formatter::default());
}

