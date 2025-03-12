// Answer 0

#[test]
fn test_make_relative_port_different() {
    let base = Url::parse("https://example.com:80/a/b/c.html").unwrap();
    let url = Url::parse("https://example.com:443/a/b/c.png").unwrap();
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_port_different_with_query() {
    let base = Url::parse("http://example.org:8080/path/to/resource").unwrap();
    let url = Url::parse("http://example.org:3000/path/to/resource?query=1").unwrap();
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_port_different_with_fragment() {
    let base = Url::parse("http://example.net:5000/folder/").unwrap();
    let url = Url::parse("http://example.net:6000/folder/file.txt#section").unwrap();
    let relative = base.make_relative(&url);
}

