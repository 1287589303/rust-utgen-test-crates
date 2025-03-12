const ACCEPT: usize = 12;
const REJECT: usize = 0;
pub(crate) fn decode_lossy<B: AsRef<[u8]>>(slice: B) -> (char, usize) {
    match decode(slice) {
        (Some(ch), size) => (ch, size),
        (None, size) => ('\u{FFFD}', size),
    }
}
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
            let ch = char::from_u32(cp).unwrap();
            return (Some(ch), i);
        } else if state == REJECT {
            return (None, core::cmp::max(1, i.saturating_sub(1)));
        }
    }
    (None, i)
}
