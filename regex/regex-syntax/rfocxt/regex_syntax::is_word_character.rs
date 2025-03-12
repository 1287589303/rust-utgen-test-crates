pub use crate::{
    error::Error, parser::{parse, Parser, ParserBuilder},
    unicode::UnicodeWordError,
};
use alloc::string::String;
#[derive(Debug)]
pub struct UnicodeWordError(());
pub fn is_word_character(c: char) -> bool {
    try_is_word_character(c).expect("unicode-perl feature must be enabled")
}
pub fn try_is_word_character(c: char) -> core::result::Result<bool, UnicodeWordError> {
    unicode::is_word_character(c)
}
