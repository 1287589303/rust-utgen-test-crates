fn pretend_parse_data_url(input: &str) -> Option<&str> {
    // Trim C0 control or space
    let left_trimmed = input.trim_start_matches(|ch| ch <= ' ');

    let mut bytes = left_trimmed.bytes();
    {
        // Ignore ASCII tabs or newlines like the URL parser would
        let mut iter = bytes
            .by_ref()
            .filter(|&byte| !matches!(byte, b'\t' | b'\n' | b'\r'));
        require!(iter.next()?.eq_ignore_ascii_case(&b'd'));
        require!(iter.next()?.eq_ignore_ascii_case(&b'a'));
        require!(iter.next()?.eq_ignore_ascii_case(&b't'));
        require!(iter.next()?.eq_ignore_ascii_case(&b'a'));
        require!(iter.next()? == b':');
    }
    let bytes_consumed = left_trimmed.len() - bytes.len();
    let after_colon = &left_trimmed[bytes_consumed..];

    // Trim C0 control or space
    Some(after_colon.trim_end_matches(|ch| ch <= ' '))
}