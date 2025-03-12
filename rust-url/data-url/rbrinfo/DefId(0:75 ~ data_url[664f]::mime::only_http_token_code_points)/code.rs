fn only_http_token_code_points(s: &str) -> bool {
    s.bytes().all(|byte| IS_HTTP_TOKEN[byte as usize])
}