pub fn protocol(url: &Url) -> &str {
    &url.as_str()[..url.scheme().len() + ":".len()]
}