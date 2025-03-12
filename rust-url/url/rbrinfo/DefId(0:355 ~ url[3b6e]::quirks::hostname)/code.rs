pub fn hostname(url: &Url) -> &str {
    url.host_str().unwrap_or("")
}