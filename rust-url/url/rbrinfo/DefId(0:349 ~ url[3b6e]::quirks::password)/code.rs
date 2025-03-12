pub fn password(url: &Url) -> &str {
    url.password().unwrap_or("")
}