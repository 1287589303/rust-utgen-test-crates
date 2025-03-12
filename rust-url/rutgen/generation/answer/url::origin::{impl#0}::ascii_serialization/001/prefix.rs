// Answer 0

#[test]
fn test_ascii_serialization_http() {
    let scheme = "http".to_owned();
    let host = Host::Domain("example.com".to_owned());
    let port = 80;
    let origin = Origin::Tuple(scheme, host, port);
    origin.ascii_serialization();
}

#[test]
fn test_ascii_serialization_https() {
    let scheme = "https".to_owned();
    let host = Host::Domain("example.com".to_owned());
    let port = 443;
    let origin = Origin::Tuple(scheme, host, port);
    origin.ascii_serialization();
}

#[test]
fn test_ascii_serialization_ws() {
    let scheme = "ws".to_owned();
    let host = Host::Domain("example.com".to_owned());
    let port = 80;
    let origin = Origin::Tuple(scheme, host, port);
    origin.ascii_serialization();
}

#[test]
fn test_ascii_serialization_wss() {
    let scheme = "wss".to_owned();
    let host = Host::Domain("example.com".to_owned());
    let port = 443;
    let origin = Origin::Tuple(scheme, host, port);
    origin.ascii_serialization();
}

#[test]
fn test_ascii_serialization_ftp() {
    let scheme = "ftp".to_owned();
    let host = Host::Domain("example.com".to_owned());
    let port = 21;
    let origin = Origin::Tuple(scheme, host, port);
    origin.ascii_serialization();
}

