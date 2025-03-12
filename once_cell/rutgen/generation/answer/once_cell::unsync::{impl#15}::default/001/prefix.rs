// Answer 0

#[test]
fn test_lazy_default_with_i32() {
    let lazy: Lazy<i32> = Lazy::default();
    let value = Lazy::into_value(lazy);
}

#[test]
fn test_lazy_default_with_string() {
    let lazy: Lazy<String> = Lazy::default();
    let value = Lazy::into_value(lazy);
}

#[test]
fn test_lazy_default_with_user_defined() {
    struct UserType {
        data: i32,
    }
    
    impl Default for UserType {
        fn default() -> Self {
            UserType { data: 0 }
        }
    }

    let lazy: Lazy<UserType> = Lazy::default();
    let value = Lazy::into_value(lazy);
}

