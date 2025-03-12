pub fn search(url: &Url) -> &str {
    trim(&url[Position::AfterPath..Position::AfterQuery])
}