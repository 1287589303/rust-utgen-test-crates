// Answer 0

#[test]
fn test_as_mut_with_u8() {
    let mut array = Array64([0u8; 64]);
    let slice: &mut [u8] = array.as_mut();
}

#[test]
fn test_as_mut_with_u32() {
    let mut array = Array64([0u32; 64]);
    let slice: &mut [u32] = array.as_mut();
}

#[test]
fn test_as_mut_with_f64() {
    let mut array = Array64([0.0f64; 64]);
    let slice: &mut [f64] = array.as_mut();
}

#[test]
fn test_as_mut_with_char() {
    let mut array = Array64(['a'; 64]);
    let slice: &mut [char] = array.as_mut();
}

#[test]
fn test_as_mut_with_string() {
    let mut array = Array64([String::from(""); 64]);
    let slice: &mut [String] = array.as_mut();
}

