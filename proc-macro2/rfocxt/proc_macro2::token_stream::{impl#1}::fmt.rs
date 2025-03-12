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
pub(crate) enum TokenTreeIter {
    Compiler(proc_macro::token_stream::IntoIter),
    Fallback(fallback::TokenTreeIter),
}
impl Debug for IntoIter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TokenStream ")?;
        f.debug_list().entries(self.clone()).finish()
    }
}
