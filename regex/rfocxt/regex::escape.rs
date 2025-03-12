pub use crate::error::Error;
pub use crate::{builders::string::*, regex::string::*, regexset::string::*};
pub fn escape(pattern: &str) -> alloc::string::String {
    regex_syntax::escape(pattern)
}
