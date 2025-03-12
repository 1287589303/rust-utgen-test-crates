#[derive(Clone, Debug)]
pub struct UnicodeWordBoundaryError(());
pub(super) fn check() -> Result<(), super::UnicodeWordBoundaryError> {
    Err(super::UnicodeWordBoundaryError::new())
}
