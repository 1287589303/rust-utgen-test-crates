use core::ops::{Index, Range, RangeFrom, RangeFull, RangeTo};
use crate::Url;
#[derive(Clone)]
pub struct Url {
    /// Syntax in pseudo-BNF:
    ///
    ///   url = scheme ":" [ hierarchical | non-hierarchical ] [ "?" query ]? [ "#" fragment ]?
    ///   non-hierarchical = non-hierarchical-path
    ///   non-hierarchical-path = /* Does not start with "/" */
    ///   hierarchical = authority? hierarchical-path
    ///   authority = "//" userinfo? host [ ":" port ]?
    ///   userinfo = username [ ":" password ]? "@"
    ///   hierarchical-path = [ "/" path-segment ]+
    serialization: String,
    scheme_end: u32,
    username_end: u32,
    host_start: u32,
    host_end: u32,
    host: HostInternal,
    port: Option<u16>,
    path_start: u32,
    query_start: Option<u32>,
    fragment_start: Option<u32>,
}
#[derive(Copy, Clone, Debug)]
pub enum Position {
    BeforeScheme,
    AfterScheme,
    BeforeUsername,
    AfterUsername,
    BeforePassword,
    AfterPassword,
    BeforeHost,
    AfterHost,
    BeforePort,
    AfterPort,
    BeforePath,
    AfterPath,
    BeforeQuery,
    AfterQuery,
    BeforeFragment,
    AfterFragment,
}
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(crate) enum HostInternal {
    None,
    Domain,
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
}
impl Url {
    #[inline]
    fn index(&self, position: Position) -> usize {
        match position {
            Position::BeforeScheme => 0,
            Position::AfterScheme => self.scheme_end as usize,
            Position::BeforeUsername => {
                if self.has_authority() {
                    self.scheme_end as usize + "://".len()
                } else {
                    debug_assert!(self.byte_at(self.scheme_end) == b':');
                    debug_assert!(
                        self.scheme_end + ":".len() as u32 == self.username_end
                    );
                    self.scheme_end as usize + ":".len()
                }
            }
            Position::AfterUsername => self.username_end as usize,
            Position::BeforePassword => {
                if self.has_authority() && self.byte_at(self.username_end) == b':' {
                    self.username_end as usize + ":".len()
                } else {
                    debug_assert!(self.username_end == self.host_start);
                    self.username_end as usize
                }
            }
            Position::AfterPassword => {
                if self.has_authority() && self.byte_at(self.username_end) == b':' {
                    debug_assert!(
                        self.byte_at(self.host_start - "@".len() as u32) == b'@'
                    );
                    self.host_start as usize - "@".len()
                } else {
                    debug_assert!(self.username_end == self.host_start);
                    self.host_start as usize
                }
            }
            Position::BeforeHost => self.host_start as usize,
            Position::AfterHost => self.host_end as usize,
            Position::BeforePort => {
                if self.port.is_some() {
                    debug_assert!(self.byte_at(self.host_end) == b':');
                    self.host_end as usize + ":".len()
                } else {
                    self.host_end as usize
                }
            }
            Position::AfterPort => {
                if let Some(port) = self.port {
                    debug_assert!(self.byte_at(self.host_end) == b':');
                    self.host_end as usize + ":".len() + count_digits(port)
                } else {
                    self.host_end as usize
                }
            }
            Position::BeforePath => self.path_start as usize,
            Position::AfterPath => {
                match (self.query_start, self.fragment_start) {
                    (Some(q), _) => q as usize,
                    (None, Some(f)) => f as usize,
                    (None, None) => self.serialization.len(),
                }
            }
            Position::BeforeQuery => {
                match (self.query_start, self.fragment_start) {
                    (Some(q), _) => {
                        debug_assert!(self.byte_at(q) == b'?');
                        q as usize + "?".len()
                    }
                    (None, Some(f)) => f as usize,
                    (None, None) => self.serialization.len(),
                }
            }
            Position::AfterQuery => {
                match self.fragment_start {
                    None => self.serialization.len(),
                    Some(f) => f as usize,
                }
            }
            Position::BeforeFragment => {
                match self.fragment_start {
                    Some(f) => {
                        debug_assert!(self.byte_at(f) == b'#');
                        f as usize + "#".len()
                    }
                    None => self.serialization.len(),
                }
            }
            Position::AfterFragment => self.serialization.len(),
        }
    }
}
fn count_digits(n: u16) -> usize {
    match n {
        0..=9 => 1,
        10..=99 => 2,
        100..=999 => 3,
        1000..=9999 => 4,
        10000..=65535 => 5,
    }
}
