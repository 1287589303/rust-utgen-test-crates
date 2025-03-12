// Answer 0

#[test]
fn test_unicode_serialization_with_non_domain_ipv4_host_default_port_excluded() {
    let scheme = "ftp";
    let host = Host::Ipv4("192.168.1.1".parse().unwrap());
    let port = 22; // port 22 is not the default for ftp
    let origin = Origin::Tuple(scheme.to_owned(), host, port);
    let result = origin.unicode_serialization();
}

#[test]
fn test_unicode_serialization_with_non_domain_ipv6_host_default_port_excluded() {
    let scheme = "ws";
    let host = Host::Ipv6("::1".parse().unwrap());
    let port = 8080; // port 8080 is not the default for ws
    let origin = Origin::Tuple(scheme.to_owned(), host, port);
    let result = origin.unicode_serialization();
}

#[test]
fn test_unicode_serialization_with_non_domain_ipv4_host_custom_port() {
    let scheme = "http"; // scheme is valid
    let host = Host::Ipv4("10.0.0.1".parse().unwrap());
    let port = 3000; // port 3000 is a valid custom port, not default for http
    let origin = Origin::Tuple(scheme.to_owned(), host, port);
    let result = origin.unicode_serialization();
}

#[test]
fn test_unicode_serialization_with_non_domain_ipv6_host_non_default_port() {
    let scheme = "https"; // scheme is valid
    let host = Host::Ipv6("2001:db8::ff00:42:8329".parse().unwrap());
    let port = 10001; // port 10001 is not the default for https
    let origin = Origin::Tuple(scheme.to_owned(), host, port);
    let result = origin.unicode_serialization();
}

