// Answer 0

#[test]
fn test_unicode_serialization_http() {
    let scheme = "http".to_owned();
    let host = Host::Domain("example.com".to_owned());
    let port = 80;
    let origin = Origin::Tuple(scheme, host, port);
    let _result = origin.unicode_serialization();
}

#[test]
fn test_unicode_serialization_https() {
    let scheme = "https".to_owned();
    let host = Host::Domain("sub.domain.com".to_owned());
    let port = 443;
    let origin = Origin::Tuple(scheme, host, port);
    let _result = origin.unicode_serialization();
}

#[test]
fn test_unicode_serialization_ftp() {
    let scheme = "ftp".to_owned();
    let host = Host::Domain("ftp.example.com".to_owned());
    let port = 21;
    let origin = Origin::Tuple(scheme, host, port);
    let _result = origin.unicode_serialization();
}

