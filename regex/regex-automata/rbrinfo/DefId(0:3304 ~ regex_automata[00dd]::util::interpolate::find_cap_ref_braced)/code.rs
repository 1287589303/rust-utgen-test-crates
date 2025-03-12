fn find_cap_ref_braced(rep: &[u8], mut i: usize) -> Option<CaptureRef<'_>> {
    assert_eq!(b'{', rep[i.checked_sub(1).unwrap()]);
    let start = i;
    while rep.get(i).map_or(false, |&b| b != b'}') {
        i += 1;
    }
    if !rep.get(i).map_or(false, |&b| b == b'}') {
        return None;
    }
    // When looking at braced names, we don't put any restrictions on the name,
    // so it's possible it could be invalid UTF-8. But a capture group name
    // can never be invalid UTF-8, so if we have invalid UTF-8, then we can
    // safely return None.
    let cap = match core::str::from_utf8(&rep[start..i]) {
        Err(_) => return None,
        Ok(cap) => cap,
    };
    Some(CaptureRef {
        cap: match cap.parse::<usize>() {
            Ok(i) => Ref::Number(i),
            Err(_) => Ref::Named(cap),
        },
        end: i + 1,
    })
}