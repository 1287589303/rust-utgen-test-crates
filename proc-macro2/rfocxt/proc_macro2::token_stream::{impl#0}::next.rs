use crate::marker::{ProcMacroAutoTraits, MARKER};
use crate::{imp, TokenTree};
use core::fmt::{self, Debug};
pub use crate::TokenStream;
#[derive(Clone)]
pub struct IntoIter {
    inner: imp::TokenTreeIter,
    _marker: ProcMacroAutoTraits,
}
#[derive(Copy, Clone)]
#[cfg_attr(
    all(procmacro2_semver_exempt, any(not(wrap_proc_macro), super_unstable)),
    derive(PartialEq, Eq)
)]
pub(crate) struct ProcMacroAutoTraits(PhantomData<Rc<()>>);
#[derive(Clone)]
pub enum TokenTree {
    /// A token stream surrounded by bracket delimiters.
    Group(Group),
    /// An identifier.
    Ident(Ident),
    /// A single punctuation character (`+`, `,`, `$`, etc.).
    Punct(Punct),
    /// A literal character (`'a'`), string (`"hello"`), number (`2.3`), etc.
    Literal(Literal),
}
#[derive(Clone)]
pub(crate) enum TokenTreeIter {
    Compiler(proc_macro::token_stream::IntoIter),
    Fallback(fallback::TokenTreeIter),
}
impl Iterator for IntoIter {
    type Item = TokenTree;
    fn next(&mut self) -> Option<TokenTree> {
        self.inner.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {}
}
