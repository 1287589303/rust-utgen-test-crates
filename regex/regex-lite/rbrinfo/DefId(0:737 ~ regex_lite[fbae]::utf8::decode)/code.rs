pub(crate) fn decode<B: AsRef<[u8]>>(slice: B) -> (Option<char>, usize) {
    let slice = slice.as_ref();
    match slice.get(0) {
        None => return (None, 0),
        Some(&b) if b <= 0x7F => return (Some(b as char), 1),
        _ => {}
    }

    let (mut state, mut cp, mut i) = (ACCEPT, 0, 0);
    while i < slice.len() {
        decode_step(&mut state, &mut cp, slice[i]);
        i += 1;

        if state == ACCEPT {
            // OK since `decode_step` guarantees that `cp` is a valid Unicode
            // scalar value in an ACCEPT state.
            //
            // We don't have to use safe code here, but do so because perf
            // isn't our primary objective in regex-lite.
            let ch = char::from_u32(cp).unwrap();
            return (Some(ch), i);
        } else if state == REJECT {
            // At this point, we always want to advance at least one byte.
            return (None, core::cmp::max(1, i.saturating_sub(1)));
        }
    }
    (None, i)
}