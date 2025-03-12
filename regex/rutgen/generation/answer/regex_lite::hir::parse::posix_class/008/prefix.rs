// Answer 0

#[test]
fn test_posix_class_lower() {
    let result = posix_class("lower");
}

#[test]
fn test_posix_class_invalid() {
    let result_alnum = posix_class("alnum");
    let result_alpha = posix_class("alpha");
    let result_ascii = posix_class("ascii");
    let result_blank = posix_class("blank");
    let result_cntrl = posix_class("cntrl");
    let result_digit = posix_class("digit");
    let result_graph = posix_class("graph");
}

