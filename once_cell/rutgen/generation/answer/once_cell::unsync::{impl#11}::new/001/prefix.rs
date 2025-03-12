// Answer 0

#[test]
fn test_lazy_new_with_string() {
    let hello = "Hello, World!".to_string();
    let lazy = Lazy::new(|| hello.to_uppercase());
    let _value = &*lazy; // Just invoke the dereference
}

#[test]
fn test_lazy_new_with_integer() {
    let lazy = Lazy::new(|| 42);
    let _value = &*lazy; // Just invoke the dereference
}

#[test]
fn test_lazy_new_with_float() {
    let lazy = Lazy::new(|| 3.14);
    let _value = &*lazy; // Just invoke the dereference
}

#[test]
fn test_lazy_new_with_struct() {
    #[derive(Debug)]
    struct MyStruct {
        value: i32,
    }
    let lazy = Lazy::new(|| MyStruct { value: 10 });
    let _value = &*lazy; // Just invoke the dereference
}

#[test]
fn test_lazy_new_with_enum() {
    #[derive(Debug)]
    enum MyEnum {
        VariantA,
        VariantB,
    }
    let lazy = Lazy::new(|| MyEnum::VariantB);
    let _value = &*lazy; // Just invoke the dereference
}

