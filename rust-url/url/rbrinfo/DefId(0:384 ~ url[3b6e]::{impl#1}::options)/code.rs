pub fn options<'a>() -> ParseOptions<'a> {
        ParseOptions {
            base_url: None,
            encoding_override: None,
            violation_fn: None,
        }
    }