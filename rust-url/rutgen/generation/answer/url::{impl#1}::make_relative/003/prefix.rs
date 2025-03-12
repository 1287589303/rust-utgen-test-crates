// Answer 0

#[test]
fn test_make_relative_different_hosts() {
    let base = Url::parse("http://example.com/path")?;
    let url = Url::parse("http://another-domain.com/another-path")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_different_ports() {
    let base = Url::parse("http://example.com:80/path")?;
    let url = Url::parse("http://example.com:8080/another-path")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_https_base() {
    let base = Url::parse("https://example.com/path")?;
    let url = Url::parse("https://another-domain.com/another-path")?;
    let relative = base.make_relative(&url);
}

#[test]
fn test_make_relative_subdomain_difference() {
    let base = Url::parse("http://sub.example.com/path")?;
    let url = Url::parse("http://example.com/another-path")?;
    let relative = base.make_relative(&url);
}

