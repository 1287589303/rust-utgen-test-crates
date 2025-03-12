use alloc::borrow::Cow;
use core::fmt;
use proc_macro2::{Ident, Span};
pub trait IdentFragment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
    fn span(&self) -> Option<Span> {
        None
    }
}
