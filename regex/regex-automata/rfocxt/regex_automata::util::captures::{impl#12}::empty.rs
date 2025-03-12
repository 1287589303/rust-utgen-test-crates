#[cfg(feature = "std")]
type CaptureNameMap = std::collections::HashMap<Arc<str>, SmallIndex>;
#[cfg(not(feature = "std"))]
type CaptureNameMap = alloc::collections::BTreeMap<Arc<str>, SmallIndex>;
use alloc::{string::String, sync::Arc, vec, vec::Vec};
use crate::util::{
    interpolate,
    primitives::{NonMaxUsize, PatternID, PatternIDError, PatternIDIter, SmallIndex},
    search::{Match, Span},
};
#[derive(Clone, Debug)]
pub struct GroupInfoPatternNames<'a> {
    it: core::slice::Iter<'a, Option<Arc<str>>>,
}
impl GroupInfoPatternNames<'static> {
    fn empty() -> GroupInfoPatternNames<'static> {
        GroupInfoPatternNames {
            it: [].iter(),
        }
    }
}
