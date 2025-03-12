// Answer 0

#[test]
fn test_posix_class_print() {
    let result = posix_class("print");
    let _ = result.unwrap(); // Ensures Ok result is returned
}

#[test]
#[should_panic]
fn test_posix_class_invalid_alnum() {
    let result = posix_class("alnum");
    let _ = result.unwrap_err(); // Should panic for expected error
}

#[test]
#[should_panic]
fn test_posix_class_invalid_alpha() {
    let result = posix_class("alpha");
    let _ = result.unwrap_err(); // Should panic for expected error
}

#[test]
#[should_panic]
fn test_posix_class_invalid_ascii() {
    let result = posix_class("ascii");
    let _ = result.unwrap_err(); // Should panic for expected error
}

#[test]
#[should_panic]
fn test_posix_class_invalid_blank() {
    let result = posix_class("blank");
    let _ = result.unwrap_err(); // Should panic for expected error
}

#[test]
#[should_panic]
fn test_posix_class_invalid_cntrl() {
    let result = posix_class("cntrl");
    let _ = result.unwrap_err(); // Should panic for expected error
}

#[test]
#[should_panic]
fn test_posix_class_invalid_digit() {
    let result = posix_class("digit");
    let _ = result.unwrap_err(); // Should panic for expected error
}

#[test]
#[should_panic]
fn test_posix_class_invalid_graph() {
    let result = posix_class("graph");
    let _ = result.unwrap_err(); // Should panic for expected error
}

#[test]
#[should_panic]
fn test_posix_class_invalid_lower() {
    let result = posix_class("lower");
    let _ = result.unwrap_err(); // Should panic for expected error
}

#[test]
#[should_panic]
fn test_posix_class_invalid_upper() {
    let result = posix_class("upper");
    let _ = result.unwrap_err(); // Should panic for expected error
}

#[test]
#[should_panic]
fn test_posix_class_invalid_word() {
    let result = posix_class("word");
    let _ = result.unwrap_err(); // Should panic for expected error
}

#[test]
#[should_panic]
fn test_posix_class_invalid_xdigit() {
    let result = posix_class("xdigit");
    let _ = result.unwrap_err(); // Should panic for expected error
}

#[test]
#[should_panic]
fn test_posix_class_unrecognized() {
    let result = posix_class("undefined");
    let _ = result.unwrap_err(); // Should panic for expected error
}

