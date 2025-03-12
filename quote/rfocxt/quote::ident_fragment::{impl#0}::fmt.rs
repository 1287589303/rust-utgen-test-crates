use alloc::borrow::Cow;
use core::fmt;
use proc_macro2::{Ident, Span};
pub trait IdentFragment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn span(&self) -> Option<Span> {
        None
    }
}
pub trait RepAsIteratorExt<'q> {
    type Iter: Iterator;
    fn quote_into_iter(&'q self) -> (Self::Iter, HasIter);
}
pub trait Spanned: private::Sealed {
    fn __span(&self) -> Span;
}
pub trait ToTokens {
    fn to_tokens(&self, tokens: &mut TokenStream);
    fn to_token_stream(&self) -> TokenStream;
    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized;
}
impl<T: IdentFragment + ?Sized> IdentFragment for &mut T {
    fn span(&self) -> Option<Span> {}
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        IdentFragment::fmt(*self, f)
    }
}
