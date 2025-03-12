// Answer 0

#[test]
fn test_valid_mime_types() {
    let valid_mime_1 = Mime::from_str("text/html");
    let valid_mime_2 = Mime::from_str("application/json");
    let valid_mime_3 = Mime::from_str("image/png");
    let valid_mime_4 = Mime::from_str("       text/plain;");
    let valid_mime_5 = Mime::from_str("application/vnd.api+json");
}

#[test]
fn test_invalid_mime_types() {
    let invalid_mime_1 = Mime::from_str("");
    let invalid_mime_2 = Mime::from_str("text/");
    let invalid_mime_3 = Mime::from_str("/html");
    let invalid_mime_4 = Mime::from_str("text/html; charset=utf-8; invalid_parameter=!");
    let invalid_mime_5 = Mime::from_str("application/json;"); // Semicolon without parameters
}

