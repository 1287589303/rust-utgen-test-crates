// Answer 0

#[test]
fn test_fmt_with_non_empty_type_and_subtype_and_empty_parameters() {
    let mime = Mime {
        type_: "text".to_owned(),
        subtype: "plain".to_owned(),
        parameters: Vec::new(),
    };
    let mut buffer = String::new();
    let _ = mime.fmt(&mut buffer);
}

#[test]
fn test_fmt_with_valid_type_and_subtype_and_empty_parameters() {
    let mime = Mime {
        type_: "application".to_owned(),
        subtype: "json".to_owned(),
        parameters: Vec::new(),
    };
    let mut buffer = String::new();
    let _ = mime.fmt(&mut buffer);
}

#[test]
fn test_fmt_with_another_valid_type_and_subtype_and_empty_parameters() {
    let mime = Mime {
        type_: "image".to_owned(),
        subtype: "png".to_owned(),
        parameters: Vec::new(),
    };
    let mut buffer = String::new();
    let _ = mime.fmt(&mut buffer);
}

