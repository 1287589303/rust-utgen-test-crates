use alloc::{
    string::{String, ToString},
    sync::Arc, vec, vec::Vec,
};
use regex_automata::{meta, nfa::thompson::WhichCaptures, util::syntax, MatchKind};
use crate::error::Error;
#[derive(Clone, Debug)]
struct Builder {
    pats: Vec<String>,
    metac: meta::Config,
    syntaxc: syntax::Config,
}
impl Default for Builder {
    fn default() -> Builder {
        let metac = meta::Config::new()
            .nfa_size_limit(Some(10 * (1 << 20)))
            .hybrid_cache_capacity(2 * (1 << 20));
        Builder {
            pats: vec![],
            metac,
            syntaxc: syntax::Config::default(),
        }
    }
}
