use crate::util::{escape::DebugByte, utf8};
#[derive(Clone, Debug)]
pub struct UnicodeWordBoundaryError(());
impl UnicodeWordBoundaryError {
    #[cfg(not(feature = "unicode-word-boundary"))]
    pub(crate) fn new() -> UnicodeWordBoundaryError {}
    pub fn check() -> Result<(), UnicodeWordBoundaryError> {
        is_word_char::check()
    }
}
pub(super) fn check() -> Result<(), super::UnicodeWordBoundaryError> {
    Err(super::UnicodeWordBoundaryError::new())
}
