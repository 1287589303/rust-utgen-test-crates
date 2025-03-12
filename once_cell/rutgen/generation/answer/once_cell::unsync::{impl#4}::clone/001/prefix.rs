// Answer 0

#[test]
fn test_clone_with_cloneable_primitive() {
    let cell = OnceCell::with_value(42);
    let cloned_cell = cell.clone();
}

#[test]
fn test_clone_with_cloneable_string() {
    let cell = OnceCell::with_value(String::from("Hello"));
    let cloned_cell = cell.clone();
}

#[test]
fn test_clone_with_cloneable_struct() {
    #[derive(Clone)]
    struct MyStruct {
        data: i32,
    }
    let cell = OnceCell::with_value(MyStruct { data: 10 });
    let cloned_cell = cell.clone();
}

#[test]
fn test_clone_with_cloneable_empty_struct() {
    #[derive(Clone)]
    struct EmptyStruct;
    let cell = OnceCell::with_value(EmptyStruct);
    let cloned_cell = cell.clone();
}

#[test]
fn test_clone_with_cloneable_nested_struct() {
    #[derive(Clone)]
    struct Outer {
        inner: Inner,
    }
    #[derive(Clone)]
    struct Inner {
        value: i32,
    }
    let cell = OnceCell::with_value(Outer { inner: Inner { value: 20 } });
    let cloned_cell = cell.clone();
}

