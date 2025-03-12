// Answer 0

#[test]
fn test_default_lazy_i32() {
    let lazy_i32 = Lazy::<i32>::default();
    let result = Lazy::into_value(lazy_i32);
}

#[test]
fn test_default_lazy_bool() {
    let lazy_bool = Lazy::<bool>::default();
    let result = Lazy::into_value(lazy_bool);
}

#[test]
fn test_default_lazy_vec() {
    let lazy_vec = Lazy::<Vec<u32>>::default();
    let result = Lazy::into_value(lazy_vec);
}

#[test]
fn test_default_lazy_empty_vec() {
    let lazy_empty_vec = Lazy::<Vec<u32>>::default();
    let result = Lazy::into_value(lazy_empty_vec);
}

#[test]
fn test_default_lazy_custom_struct() {
    #[derive(Default)]
    struct CustomStruct {
        value: i32,
    }
    
    let lazy_custom_struct = Lazy::<CustomStruct>::default();
    let result = Lazy::into_value(lazy_custom_struct);
}

#[test]
fn test_default_lazy_custom_struct_empty() {
    #[derive(Default)]
    struct EmptyStruct {
        name: String,
    }
    
    let lazy_empty_struct = Lazy::<EmptyStruct>::default();
    let result = Lazy::into_value(lazy_empty_struct);
}

#[test]
fn test_default_lazy_hashmap() {
    use std::collections::HashMap;

    let lazy_hashmap = Lazy::<HashMap<String, i32>>::default();
    let result = Lazy::into_value(lazy_hashmap);
}

