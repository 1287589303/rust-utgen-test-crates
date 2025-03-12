fn is_passthrough_ascii_label(label: &[u8]) -> bool {
    // XXX if we aren't performing _CheckHyphens_, this could
    // check for "xn--" and pass through YouTube CDN node names.
    if label.len() >= 4 && label[2] == b'-' && label[3] == b'-' {
        return false;
    }
    if let Some((&first, tail)) = label.split_first() {
        // We need to check the first and last character
        // more strictly in case this turns out to be a
        // label in a bidi domain name. This has the side
        // effect that this function only accepts labels
        // that also conform to the STD3 rules.
        //
        // XXX: If we are in the fail-fast mode (i.e. we don't need
        // to be able to overwrite anything with U+FFFD), we could
        // merely record that we've seen a digit here and error out
        // if we later discover that the domain name is a bidi
        // domain name.
        if !in_inclusive_range8(first, b'a', b'z') {
            return false;
        }
        for &b in tail {
            // If we used LDH_MASK, we'd have to check
            // the bytes for the ASCII range anyhow.
            if in_inclusive_range8(b, b'a', b'z') {
                continue;
            }
            if in_inclusive_range8(b, b'0', b'9') {
                continue;
            }
            if b == b'-' {
                continue;
            }
            return false;
        }
        label.last() != Some(&b'-')
    } else {
        // empty
        true
    }
}