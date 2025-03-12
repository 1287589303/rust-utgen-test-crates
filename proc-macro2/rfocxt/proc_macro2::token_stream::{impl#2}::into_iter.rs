use crate::marker::{ProcMacroAutoTraits, MARKER};
use crate::{imp, TokenTree};
use core::fmt::{self, Debug};
pub use crate::TokenStream;
#[derive(Clone)]
pub struct TokenStream {
    inner: imp::TokenStream,
    _marker: ProcMacroAutoTraits,
}
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
pub(crate) struct TokenStream {
    inner: RcVec<TokenTree>,
}
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
pub(crate) enum TokenStream {
    Compiler(DeferredTokenStream),
    Fallback(fallback::TokenStream),
}
impl IntoIterator for TokenStream {
    type Item = TokenTree;
    type IntoIter = IntoIter;
    fn into_iter(self) -> IntoIter {
        IntoIter {
            inner: self.inner.into_iter(),
            _marker: MARKER,
        }
    }
}
