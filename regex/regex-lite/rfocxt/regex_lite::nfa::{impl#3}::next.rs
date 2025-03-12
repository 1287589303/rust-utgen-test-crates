pub(crate) type StateID = u32;
#[cfg(feature = "std")]
type CaptureNameMap = std::collections::HashMap<Arc<str>, u32>;
#[cfg(not(feature = "std"))]
type CaptureNameMap = alloc::collections::BTreeMap<Arc<str>, u32>;
use core::{cell::RefCell, mem::size_of};
use alloc::{string::String, sync::Arc, vec, vec::Vec};
use crate::{
    error::Error, hir::{self, Hir, HirKind},
    int::U32,
};
#[derive(Clone, Debug)]
pub(crate) struct CaptureNames<'a> {
    it: core::slice::Iter<'a, Option<Arc<str>>>,
}
impl<'a> Iterator for CaptureNames<'a> {
    type Item = Option<&'a str>;
    fn next(&mut self) -> Option<Option<&'a str>> {
        self.it.next().map(|n| n.as_deref())
    }
}
