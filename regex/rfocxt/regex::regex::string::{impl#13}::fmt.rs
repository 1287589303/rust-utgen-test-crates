pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{error::Error, RegexBuilder};
pub struct Captures<'h> {
    haystack: &'h str,
    caps: captures::Captures,
    static_captures_len: Option<usize>,
}
pub struct Captures<'h> {
    haystack: &'h [u8],
    caps: captures::Captures,
    static_captures_len: Option<usize>,
}
impl<'h> core::fmt::Debug for Captures<'h> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        /// A little helper type to provide a nice map-like debug
        /// representation for our capturing group spans.
        ///
        /// regex-automata has something similar, but it includes the pattern
        /// ID in its debug output, which is confusing. It also doesn't include
        /// that strings that match because a regex-automata `Captures` doesn't
        /// borrow the haystack.
        struct CapturesDebugMap<'a> {
            caps: &'a Captures<'a>,
        }
        impl<'a> core::fmt::Debug for CapturesDebugMap<'a> {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                let mut map = f.debug_map();
                let names = self.caps.caps.group_info().pattern_names(PatternID::ZERO);
                for (group_index, maybe_name) in names.enumerate() {
                    let key = Key(group_index, maybe_name);
                    match self.caps.get(group_index) {
                        None => map.entry(&key, &None::<()>),
                        Some(mat) => map.entry(&key, &Value(mat)),
                    };
                }
                map.finish()
            }
        }
        struct Key<'a>(usize, Option<&'a str>);
        impl<'a> core::fmt::Debug for Key<'a> {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", self.0)?;
                if let Some(name) = self.1 {
                    write!(f, "/{:?}", name)?;
                }
                Ok(())
            }
        }
        struct Value<'a>(Match<'a>);
        impl<'a> core::fmt::Debug for Value<'a> {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}..{}/{:?}", self.0.start(), self.0.end(), self.0.as_str())
            }
        }
        f.debug_tuple("Captures").field(&CapturesDebugMap { caps: self }).finish()
    }
}
