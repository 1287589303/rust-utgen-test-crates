pub fn host(url: &Url) -> &str {
    &url[Position::BeforeHost..Position::AfterPort]
}