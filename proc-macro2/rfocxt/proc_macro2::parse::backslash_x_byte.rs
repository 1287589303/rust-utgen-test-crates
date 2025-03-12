type PResult<'a, O> = Result<(Cursor<'a>, O), Reject>;
use crate::fallback::{
    self, is_ident_continue, is_ident_start, Group, Ident, LexError, Literal, Span,
    TokenStream, TokenStreamBuilder,
};
use crate::{Delimiter, Punct, Spacing, TokenTree};
use core::char;
use core::str::{Bytes, CharIndices, Chars};
const ERROR: &str = "(/*ERROR*/)";
pub(crate) struct Reject;
fn backslash_x_byte<I>(chars: &mut I) -> Result<(), Reject>
where
    I: Iterator<Item = (usize, u8)>,
{
    next_ch!(chars @ b'0'..= b'9' | b'a'..= b'f' | b'A'..= b'F');
    next_ch!(chars @ b'0'..= b'9' | b'a'..= b'f' | b'A'..= b'F');
    Ok(())
}
