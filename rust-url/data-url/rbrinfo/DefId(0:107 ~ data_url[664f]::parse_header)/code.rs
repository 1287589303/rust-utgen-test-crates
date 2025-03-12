fn parse_header(from_colon_to_comma: &str) -> (mime::Mime, bool) {
    // "Strip leading and trailing ASCII whitespace"
    //     \t, \n, and \r would have been filtered by the URL parser
    //     \f percent-encoded by the URL parser
    //     space is the only remaining ASCII whitespace
    let trimmed = from_colon_to_comma.trim_matches(|c| matches!(c, ' ' | '\t' | '\n' | '\r'));

    let without_base64_suffix = remove_base64_suffix(trimmed);
    let base64 = without_base64_suffix.is_some();
    let mime_type = without_base64_suffix.unwrap_or(trimmed);

    let mut string = String::new();
    if mime_type.starts_with(';') {
        string.push_str("text/plain")
    }
    let mut in_query = false;
    for byte in mime_type.bytes() {
        match byte {
            // Ignore ASCII tabs or newlines like the URL parser would
            b'\t' | b'\n' | b'\r' => continue,

            // https://url.spec.whatwg.org/#c0-control-percent-encode-set
            b'\0'..=b'\x1F' | b'\x7F'..=b'\xFF' => percent_encode(byte, &mut string),

            // Bytes other than the C0 percent-encode set that are percent-encoded
            // by the URL parser in the query state.
            // '#' is also in that list but cannot occur here
            // since it indicates the start of the URL’s fragment.
            b' ' | b'"' | b'<' | b'>' if in_query => percent_encode(byte, &mut string),

            b'?' => {
                in_query = true;
                string.push('?')
            }

            // Printable ASCII
            _ => string.push(byte as char),
        }
    }

    // FIXME: does Mime::from_str match the MIME Sniffing Standard’s parsing algorithm?
    // <https://mimesniff.spec.whatwg.org/#parse-a-mime-type>
    let mime_type = string.parse().unwrap_or_else(|_| mime::Mime {
        type_: String::from("text"),
        subtype: String::from("plain"),
        parameters: vec![(String::from("charset"), String::from("US-ASCII"))],
    });

    (mime_type, base64)
}