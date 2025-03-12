// Answer 0

#[test]
fn test_get_parameter_with_existing_parameter() {
    let mime = Mime {
        type_: "text".to_owned(),
        subtype: "plain".to_owned(),
        parameters: vec![("charset".to_owned(), "utf-8".to_owned())],
    };
    mime.get_parameter("charset");
}

#[test]
fn test_get_parameter_with_non_existing_parameter() {
    let mime = Mime {
        type_: "text".to_owned(),
        subtype: "plain".to_owned(),
        parameters: vec![("charset".to_owned(), "utf-8".to_owned())],
    };
    mime.get_parameter("non-existing");
}

#[test]
fn test_get_parameter_with_case_sensitive_match() {
    let mime = Mime {
        type_: "text".to_owned(),
        subtype: "html".to_owned(),
        parameters: vec![("CHARSET".to_owned(), "utf-8".to_owned())],
    };
    mime.get_parameter("charset");
}

#[test]
fn test_get_parameter_with_empty_parameters() {
    let mime = Mime {
        type_: "application".to_owned(),
        subtype: "json".to_owned(),
        parameters: Vec::new(),
    };
    mime.get_parameter("charset");
}

#[test]
fn test_get_parameter_with_long_parameter_name() {
    let long_name = "a".repeat(255);
    let mime = Mime {
        type_: "image".to_owned(),
        subtype: "png".to_owned(),
        parameters: vec![(long_name.clone(), "value".to_owned())],
    };
    mime.get_parameter(&long_name);
} 

#[test]
fn test_get_parameter_with_empty_name() {
    let mime = Mime {
        type_: "text".to_owned(),
        subtype: "plain".to_owned(),
        parameters: vec![("charset".to_owned(), "utf-8".to_owned())],
    };
    mime.get_parameter("");
}

