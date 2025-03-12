pub fn hash(url: &Url) -> &str {
    trim(&url[Position::AfterQuery..])
}