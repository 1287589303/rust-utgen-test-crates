fn remove_base64_suffix(s: &str) -> Option<&str> {
    let mut bytes = s.bytes();
    {
        // Ignore ASCII tabs or newlines like the URL parser would
        let iter = bytes
            .by_ref()
            .filter(|&byte| !matches!(byte, b'\t' | b'\n' | b'\r'));

        // Search from the end
        let mut iter = iter.rev();

        require!(iter.next()? == b'4');
        require!(iter.next()? == b'6');
        require!(iter.next()?.eq_ignore_ascii_case(&b'e'));
        require!(iter.next()?.eq_ignore_ascii_case(&b's'));
        require!(iter.next()?.eq_ignore_ascii_case(&b'a'));
        require!(iter.next()?.eq_ignore_ascii_case(&b'b'));
        require!(iter.skip_while(|&byte| byte == b' ').next()? == b';');
    }
    Some(&s[..bytes.len()])
}