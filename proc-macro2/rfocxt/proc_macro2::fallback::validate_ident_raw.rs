pub(crate) type TokenTreeIter = RcVecIntoIter<TokenTree>;
#[cfg(wrap_proc_macro)]
use crate::imp;
#[cfg(span_locations)]
use crate::location::LineColumn;
use crate::parse::{self, Cursor};
use crate::rcvec::{RcVec, RcVecBuilder, RcVecIntoIter, RcVecMut};
use crate::{Delimiter, Spacing, TokenTree};
#[cfg(all(span_locations, not(fuzzing)))]
use alloc::collections::BTreeMap;
#[cfg(all(span_locations, not(fuzzing)))]
use core::cell::RefCell;
#[cfg(span_locations)]
use core::cmp;
use core::fmt::{self, Debug, Display, Write};
use core::mem::ManuallyDrop;
#[cfg(span_locations)]
use core::ops::Range;
use core::ops::RangeBounds;
use core::ptr;
use core::str;
#[cfg(feature = "proc-macro")]
use core::str::FromStr;
use std::ffi::CStr;
#[cfg(wrap_proc_macro)]
use std::panic;
#[cfg(procmacro2_semver_exempt)]
use std::path::PathBuf;
#[track_caller]
fn validate_ident_raw(string: &str) {
    validate_ident(string);
    match string {
        "_" | "super" | "self" | "Self" | "crate" => {
            panic!("`r#{}` cannot be a raw identifier", string);
        }
        _ => {}
    }
}
#[track_caller]
fn validate_ident(string: &str) {
    if string.is_empty() {
        panic!("Ident is not allowed to be empty; use Option<Ident>");
    }
    if string.bytes().all(|digit| b'0' <= digit && digit <= b'9') {
        panic!("Ident cannot be a number; use Literal instead");
    }
    fn ident_ok(string: &str) -> bool {
        let mut chars = string.chars();
        let first = chars.next().unwrap();
        if !is_ident_start(first) {
            return false;
        }
        for ch in chars {
            if !is_ident_continue(ch) {
                return false;
            }
        }
        true
    }
    if !ident_ok(string) {
        panic!("{:?} is not a valid Ident", string);
    }
}
