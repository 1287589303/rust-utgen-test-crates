// Answer 0

#[test]
fn test_default_values_empty_iter() {
    let _: Values<i32, i32> = Values::default();
}

#[test]
fn test_default_values_empty_iter_with_strings() {
    let _: Values<String, String> = Values::default();
}

#[test]
fn test_default_values_empty_iter_with_structs() {
    #[derive(Debug)]
    struct MyStruct {
        field: i32,
    }
    let _: Values<MyStruct, MyStruct> = Values::default();
}

#[test]
fn test_default_values_empty_iter_with_floats() {
    let _: Values<f32, f32> = Values::default();
}

#[test]
fn test_default_values_empty_iter_with_complex_types() {
    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }
    let _: Values<Complex, Complex> = Values::default();
}

