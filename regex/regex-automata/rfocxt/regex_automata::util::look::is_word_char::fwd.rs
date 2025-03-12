#[derive(Clone, Debug)]
pub struct UnicodeWordBoundaryError(());
#[cfg_attr(feature = "perf-inline", inline(always))]
pub(super) fn fwd(
    _bytes: &[u8],
    _at: usize,
) -> Result<bool, super::UnicodeWordBoundaryError> {
    Err(super::UnicodeWordBoundaryError::new())
}
#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn decode(bytes: &[u8]) -> Option<Result<char, u8>> {
    if bytes.is_empty() {
        return None;
    }
    let len = match len(bytes[0]) {
        None => return Some(Err(bytes[0])),
        Some(len) if len > bytes.len() => return Some(Err(bytes[0])),
        Some(1) => return Some(Ok(char::from(bytes[0]))),
        Some(len) => len,
    };
    match core::str::from_utf8(&bytes[..len]) {
        Ok(s) => Some(Ok(s.chars().next().unwrap())),
        Err(_) => Some(Err(bytes[0])),
    }
}
