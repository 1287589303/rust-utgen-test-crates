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
fn backslash_x_nonzero<I>(chars: &mut I) -> Result<(), Reject>
where
    I: Iterator<Item = (usize, char)>,
{
    let first = next_ch!(chars @ '0'..='9' | 'a'..='f' | 'A'..='F');
    let second = next_ch!(chars @ '0'..='9' | 'a'..='f' | 'A'..='F');
    if first == '0' && second == '0' { Err(Reject) } else { Ok(()) }
}
