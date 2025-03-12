use crate::parser::{default_port, Context, Input, Parser, SchemeType};
use crate::{Host, ParseError, Position, Url};
use alloc::string::String;
use alloc::string::ToString;
fn trim(s: &str) -> &str {
    if s.len() == 1 { "" } else { s }
}
