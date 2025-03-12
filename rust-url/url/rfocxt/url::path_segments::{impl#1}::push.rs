use crate::parser::{self, to_u32, SchemeType};
use crate::Url;
use alloc::string::String;
use core::str;
#[derive(Debug)]
pub struct PathSegmentsMut<'a> {
    url: &'a mut Url,
    after_first_slash: usize,
    after_path: String,
    old_after_path_position: u32,
}
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
impl PathSegmentsMut<'_> {
    pub fn clear(&mut self) -> &mut Self {
        self.url.serialization.truncate(self.after_first_slash);
        self
    }
    pub fn pop_if_empty(&mut self) -> &mut Self {
        if self.after_first_slash >= self.url.serialization.len() {
            return self;
        }
        if self.url.serialization[self.after_first_slash..].ends_with('/') {
            self.url.serialization.pop();
        }
        self
    }
    pub fn pop(&mut self) -> &mut Self {
        if self.after_first_slash >= self.url.serialization.len() {
            return self;
        }
        let last_slash = self
            .url
            .serialization[self.after_first_slash..]
            .rfind('/')
            .unwrap_or(0);
        self.url.serialization.truncate(self.after_first_slash + last_slash);
        self
    }
    pub fn push(&mut self, segment: &str) -> &mut Self {
        self.extend(Some(segment))
    }
    pub fn extend<I>(&mut self, segments: I) -> &mut Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let scheme_type = SchemeType::from(self.url.scheme());
        let path_start = self.url.path_start as usize;
        self.url
            .mutate(|parser| {
                parser.context = parser::Context::PathSegmentSetter;
                for segment in segments {
                    let segment = segment.as_ref();
                    if matches!(segment, "." | "..") {
                        continue;
                    }
                    if parser.serialization.len() > path_start + 1
                        || parser.serialization.len() == path_start
                    {
                        parser.serialization.push('/');
                    }
                    let mut has_host = true;
                    parser
                        .parse_path(
                            scheme_type,
                            &mut has_host,
                            path_start,
                            parser::Input::new_no_trim(segment),
                        );
                }
            });
        self
    }
}
