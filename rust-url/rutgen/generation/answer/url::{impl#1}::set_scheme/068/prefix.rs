// Answer 0

#[test]
fn test_set_scheme_invalid_characters() {
    let mut url = Url::parse("ftp://username:password@example.net:8080/path")?;
    let result = url.set_scheme("inval!d");
}

#[test]
fn test_set_scheme_special_to_special_invalid() {
    let mut url = Url::parse("ftp://username:password@example.net:8080/path")?;
    let result = url.set_scheme("http");
}

#[test]
fn test_set_scheme_file_with_credentials() {
    let mut url = Url::parse("ftp://username:password@example.net:8080/path")?;
    let result = url.set_scheme("file");
}

#[test]
fn test_set_scheme_file_with_host() {
    let mut url = Url::parse("ftp://username:password@example.net:8080/path")?;
    let result = url.set_scheme("file");
}

