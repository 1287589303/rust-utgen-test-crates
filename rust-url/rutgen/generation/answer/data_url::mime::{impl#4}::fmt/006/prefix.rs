// Answer 0

#[test]
fn test_fmt_valid_cases() {
    let mime = Mime {
        type_: "text".to_owned(),
        subtype: "plain".to_owned(),
        parameters: vec![("charset".to_owned(), "utf-8".to_owned())],
    };
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", mime);
}

#[test]
#[should_panic]
fn test_fmt_invalid_equals() {
    let mime = Mime {
        type_: "text".to_owned(),
        subtype: "plain".to_owned(),
        parameters: vec![("charset".to_owned(), "utf-8;invalid".to_owned())],
    };
    let mut buffer = String::new();
    let _ = write!(&mut buffer, "{}", mime);
}

