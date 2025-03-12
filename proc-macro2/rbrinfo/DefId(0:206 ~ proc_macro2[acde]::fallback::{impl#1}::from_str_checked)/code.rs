pub(crate) fn from_str_checked(src: &str) -> Result<Self, LexError> {
        // Create a dummy file & add it to the source map
        let mut cursor = get_cursor(src);

        // Strip a byte order mark if present
        const BYTE_ORDER_MARK: &str = "\u{feff}";
        if cursor.starts_with(BYTE_ORDER_MARK) {
            cursor = cursor.advance(BYTE_ORDER_MARK.len());
        }

        parse::token_stream(cursor)
    }