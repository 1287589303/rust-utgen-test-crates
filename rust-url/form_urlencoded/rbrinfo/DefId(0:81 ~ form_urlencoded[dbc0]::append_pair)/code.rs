fn append_pair(
    string: &mut String,
    start_position: usize,
    encoding: EncodingOverride<'_>,
    name: &str,
    value: &str,
) {
    append_separator_if_needed(string, start_position);
    append_encoded(name, string, encoding);
    string.push('=');
    append_encoded(value, string, encoding);
}