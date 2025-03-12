pub fn origin(url: &Url) -> String {
    url.origin().ascii_serialization()
}