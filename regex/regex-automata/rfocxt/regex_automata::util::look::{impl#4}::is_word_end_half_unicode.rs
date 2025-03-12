use crate::util::{escape::DebugByte, utf8};
#[derive(Clone, Debug)]
pub struct LookMatcher {
    lineterm: DebugByte,
}
#[derive(Clone, Copy)]
pub struct DebugByte(pub u8);
#[derive(Clone, Debug)]
pub struct UnicodeWordBoundaryError(());
impl LookMatcher {
    pub fn new() -> LookMatcher {}
    pub fn set_line_terminator(&mut self, byte: u8) -> &mut LookMatcher {}
    pub fn get_line_terminator(&self) -> u8 {}
    #[inline]
    pub fn matches(&self, look: Look, haystack: &[u8], at: usize) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn matches_inline(&self, look: Look, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn matches_set(&self, set: LookSet, haystack: &[u8], at: usize) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn matches_set_inline(
        &self,
        set: LookSet,
        haystack: &[u8],
        at: usize,
    ) -> bool {}
    #[cfg(feature = "alloc")]
    pub(crate) fn add_to_byteset(
        &self,
        look: Look,
        set: &mut crate::util::alphabet::ByteClassSet,
    ) {}
    #[inline]
    pub fn is_start(&self, _haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_end(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_start_lf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_end_lf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_start_crlf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_end_crlf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_ascii_negate(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_unicode_negate(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_start_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_end_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_start_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_end_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_start_half_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_end_half_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_start_half_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_end_half_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {
        let word_after = at < haystack.len()
            && match utf8::decode(&haystack[at..]) {
                None | Some(Err(_)) => return Ok(false),
                Some(Ok(_)) => is_word_char::fwd(haystack, at)?,
            };
        Ok(!word_after)
    }
}
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
