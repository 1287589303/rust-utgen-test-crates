pub fn file_host(input: Input) -> ParseResult<(bool, String, Input)> {
        // Undo the Input abstraction here to avoid allocating in the common case
        // where the host part of the input does not contain any tab or newline
        let input_str = input.chars.as_str();
        let mut has_ignored_chars = false;
        let mut non_ignored_chars = 0;
        let mut bytes = 0;
        for c in input_str.chars() {
            match c {
                '/' | '\\' | '?' | '#' => break,
                ascii_tab_or_new_line_pattern!() => has_ignored_chars = true,
                _ => non_ignored_chars += 1,
            }
            bytes += c.len_utf8();
        }
        let replaced: String;
        let host_str;
        let mut remaining = input.clone();
        {
            let host_input = remaining.by_ref().take(non_ignored_chars);
            if has_ignored_chars {
                replaced = host_input.collect();
                host_str = &*replaced
            } else {
                for _ in host_input {}
                host_str = &input_str[..bytes]
            }
        }
        if is_windows_drive_letter(host_str) {
            return Ok((false, "".to_string(), input));
        }
        Ok((true, host_str.to_string(), remaining))
    }