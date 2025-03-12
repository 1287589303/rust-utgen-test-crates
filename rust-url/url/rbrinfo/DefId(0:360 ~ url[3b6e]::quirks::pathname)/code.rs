pub fn pathname(url: &Url) -> &str {
    url.path()
}