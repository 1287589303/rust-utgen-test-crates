// Answer 0

#[test]
fn test_posix_class_word() {
    let result = posix_class("word");
}

#[test]
fn test_posix_class_invalid() {
    let result_invalid_alnum = posix_class("alnum");
    let result_invalid_alpha = posix_class("alpha");
    let result_invalid_ascii = posix_class("ascii");
    let result_invalid_blank = posix_class("blank");
    let result_invalid_cntrl = posix_class("cntrl");
    let result_invalid_digit = posix_class("digit");
    let result_invalid_graph = posix_class("graph");
    let result_invalid_lower = posix_class("lower");
    let result_invalid_print = posix_class("print");
    let result_invalid_punct = posix_class("punct");
    let result_invalid_space = posix_class("space");
    let result_invalid_upper = posix_class("upper");
}

