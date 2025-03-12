pub fn port(url: &Url) -> &str {
    &url[Position::BeforePort..Position::AfterPort]
}