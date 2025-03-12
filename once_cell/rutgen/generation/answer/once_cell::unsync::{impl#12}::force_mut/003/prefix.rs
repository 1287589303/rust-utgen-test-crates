// Answer 0

#[test]
fn test_force_mut_with_initialized_value() {
    struct InitFunc;

    impl InitFunc {
        fn call() -> i32 {
            42
        }
    }

    let mut lazy = Lazy {
        cell: OnceCell::with_value(42),
        init: Cell::new(Some(InitFunc::call)),
    };

    let result = Lazy::force_mut(&mut lazy);
}

#[test]
fn test_force_mut_with_different_initialized_value() {
    struct InitFunc;

    impl InitFunc {
        fn call() -> String {
            String::from("Hello")
        }
    }

    let mut lazy = Lazy {
        cell: OnceCell::with_value(String::from("Hello")),
        init: Cell::new(Some(InitFunc::call)),
    };
    
    let result = Lazy::force_mut(&mut lazy);
} 

#[test]
fn test_force_mut_with_float_initialized_value() {
    struct InitFunc;

    impl InitFunc {
        fn call() -> f64 {
            3.14
        }
    }

    let mut lazy = Lazy {
        cell: OnceCell::with_value(3.14),
        init: Cell::new(Some(InitFunc::call)),
    };

    let result = Lazy::force_mut(&mut lazy);
} 

#[test]
fn test_force_mut_with_struct_initialized_value() {
    #[derive(Debug)]
    struct MyStruct {
        value: i32,
    }

    struct InitFunc;

    impl InitFunc {
        fn call() -> MyStruct {
            MyStruct { value: 100 }
        }
    }

    let mut lazy = Lazy {
        cell: OnceCell::with_value(MyStruct { value: 100 }),
        init: Cell::new(Some(InitFunc::call)),
    };

    let result = Lazy::force_mut(&mut lazy);
} 

#[test]
fn test_force_mut_with_empty_string_initialized_value() {
    struct InitFunc;

    impl InitFunc {
        fn call() -> String {
            String::new()
        }
    }

    let mut lazy = Lazy {
        cell: OnceCell::with_value(String::new()),
        init: Cell::new(Some(InitFunc::call)),
    };

    let result = Lazy::force_mut(&mut lazy);
}

