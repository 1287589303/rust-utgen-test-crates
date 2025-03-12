// Answer 0

#[test]
fn test_random_u8() {
    let result: u8 = rand::random();
}

#[test]
fn test_random_f32() {
    let result: f32 = rand::random();
}

#[test]
fn test_random_f64() {
    let result: f64 = rand::random();
}

#[test]
fn test_random_bool() {
    let result: bool = rand::random();
}

#[test]
fn test_random_multiple_calls() {
    for _ in 0..10 {
        let _result_u8: u8 = rand::random();
        let _result_f32: f32 = rand::random();
        let _result_f64: f64 = rand::random();
        let _result_bool: bool = rand::random();
    }
}

