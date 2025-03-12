pub(crate) fn find_fwd(
    needles: &[u8],
    haystack: &[u8],
    at: usize,
) -> Option<usize> {
    let bs = needles;
    let i = match needles.len() {
        1 => memchr::memchr(bs[0], &haystack[at..])?,
        2 => memchr::memchr2(bs[0], bs[1], &haystack[at..])?,
        3 => memchr::memchr3(bs[0], bs[1], bs[2], &haystack[at..])?,
        0 => panic!("cannot find with empty needles"),
        n => panic!("invalid needles length: {}", n),
    };
    Some(at + i)
}