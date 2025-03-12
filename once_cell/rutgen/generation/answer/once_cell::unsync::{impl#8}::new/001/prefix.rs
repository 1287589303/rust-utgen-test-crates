// Answer 0

#[test]
fn test_once_cell_new_i32() {
    let cell: OnceCell<i32> = OnceCell::new();
}

#[test]
fn test_once_cell_new_f64() {
    let cell: OnceCell<f64> = OnceCell::new();
}

#[test]
fn test_once_cell_new_char() {
    let cell: OnceCell<char> = OnceCell::new();
}

#[test]
fn test_once_cell_new_unit() {
    let cell: OnceCell<()> = OnceCell::new();
}

#[test]
fn test_once_cell_new_struct() {
    struct MyStruct;
    let cell: OnceCell<MyStruct> = OnceCell::new();
}

#[test]
fn test_once_cell_new_enum() {
    enum MyEnum {
        Variant1,
        Variant2,
    }
    let cell: OnceCell<MyEnum> = OnceCell::new();
}

