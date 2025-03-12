// Answer 0

#[test]
fn test_authority_with_userinfo_and_port() {
    let url = Url::parse("https://user:password@domain.com:8080/path").unwrap();
    let authority = url.authority();
}

#[test]
fn test_authority_without_userinfo_and_with_port() {
    let url = Url::parse("http://example.com:8000/path").unwrap();
    let authority = url.authority();
}

#[test]
fn test_authority_with_username_only() {
    let url = Url::parse("irc://user@xn--example-dk0b.com:6667/path").unwrap();
    let authority = url.authority();
}

#[test]
fn test_authority_with_punycode_domain() {
    let url = Url::parse("http://xn--lex-8ka.xn--p1ai:80/path").unwrap();
    let authority = url.authority();
}

#[test]
fn test_authority_with_empty_userinfo() {
    let url = Url::parse("ftp://:@domain.com:21/path").unwrap();
    let authority = url.authority();
}

