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
#[derive(Clone, Copy, Debug)]
pub(crate) struct Config {
    pub(crate) size_limit: Option<usize>,
}
impl Default for Config {
    fn default() -> Config {
        Config {
            size_limit: Some(10 * (1 << 20)),
        }
    }
}
