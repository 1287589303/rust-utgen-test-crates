fn valid_value(s: &str) -> bool {
    s.chars().all(|c| {
        // <https://mimesniff.spec.whatwg.org/#http-quoted-string-token-code-point>
        matches!(c, '\t' | ' '..='~' | '\u{80}'..='\u{FF}')
    })
}