use alloc::borrow::Cow;
use alloc::string::String;
pub use uts46::AsciiDenyList;
use uts46::Uts46;
#[allow(deprecated)]
pub use crate::deprecated::{Config, Idna};
pub(crate) trait PunycodeCodeUnit {
    fn is_delimiter(&self) -> bool;
    fn is_ascii(&self) -> bool;
    fn digit(&self) -> Option<u32>;
    fn char(&self) -> char;
    fn char_ascii_lower_case(&self) -> char;
}
#[derive(Default, Debug)]
#[non_exhaustive]
pub struct Errors {}
impl From<Errors> for Result<(), Errors> {
    fn from(e: Errors) -> Result<(), Errors> {
        Err(e)
    }
}
