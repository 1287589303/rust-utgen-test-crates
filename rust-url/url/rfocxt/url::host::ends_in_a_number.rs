use crate::net::{Ipv4Addr, Ipv6Addr};
use alloc::borrow::Cow;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use core::cmp;
use core::fmt::{self, Formatter};
use percent_encoding::{percent_decode, utf8_percent_encode, CONTROLS};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use crate::parser::{ParseError, ParseResult};
fn ends_in_a_number(input: &str) -> bool {
    let mut parts = input.rsplit('.');
    let last = parts.next().unwrap();
    let last = if last.is_empty() {
        if let Some(last) = parts.next() {
            last
        } else {
            return false;
        }
    } else {
        last
    };
    if !last.is_empty() && last.as_bytes().iter().all(|c| c.is_ascii_digit()) {
        return true;
    }
    parse_ipv4number(last).is_ok()
}
fn parse_ipv4number(mut input: &str) -> Result<Option<u32>, ()> {
    if input.is_empty() {
        return Err(());
    }
    let mut r = 10;
    if input.starts_with("0x") || input.starts_with("0X") {
        input = &input[2..];
        r = 16;
    } else if input.len() >= 2 && input.starts_with('0') {
        input = &input[1..];
        r = 8;
    }
    if input.is_empty() {
        return Ok(Some(0));
    }
    let valid_number = match r {
        8 => input.as_bytes().iter().all(|c| (b'0'..=b'7').contains(c)),
        10 => input.as_bytes().iter().all(|c| c.is_ascii_digit()),
        16 => input.as_bytes().iter().all(|c| c.is_ascii_hexdigit()),
        _ => false,
    };
    if !valid_number {
        return Err(());
    }
    match u32::from_str_radix(input, r) {
        Ok(num) => Ok(Some(num)),
        Err(_) => Ok(None),
    }
}
