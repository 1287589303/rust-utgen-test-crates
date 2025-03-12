use alloc::{string::String, vec::Vec};
use crate::util::memchr::memchr;
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct CaptureRef<'a> {
    cap: Ref<'a>,
    end: usize,
}
pub fn bytes(
    mut replacement: &[u8],
    mut append: impl FnMut(usize, &mut Vec<u8>),
    mut name_to_index: impl FnMut(&str) -> Option<usize>,
    dst: &mut Vec<u8>,
) {
    while !replacement.is_empty() {
        match memchr(b'$', replacement) {
            None => break,
            Some(i) => {
                dst.extend_from_slice(&replacement[..i]);
                replacement = &replacement[i..];
            }
        }
        if replacement.get(1).map_or(false, |&b| b == b'$') {
            dst.push(b'$');
            replacement = &replacement[2..];
            continue;
        }
        debug_assert!(! replacement.is_empty());
        let cap_ref = match find_cap_ref(replacement) {
            Some(cap_ref) => cap_ref,
            None => {
                dst.push(b'$');
                replacement = &replacement[1..];
                continue;
            }
        };
        replacement = &replacement[cap_ref.end..];
        match cap_ref.cap {
            Ref::Number(i) => append(i, dst),
            Ref::Named(name) => {
                if let Some(i) = name_to_index(name) {
                    append(i, dst);
                }
            }
        }
    }
    dst.extend_from_slice(replacement);
}
fn find_cap_ref(replacement: &[u8]) -> Option<CaptureRef<'_>> {
    let mut i = 0;
    let rep: &[u8] = replacement;
    if rep.len() <= 1 || rep[0] != b'$' {
        return None;
    }
    i += 1;
    if rep[i] == b'{' {
        return find_cap_ref_braced(rep, i + 1);
    }
    let mut cap_end = i;
    while rep.get(cap_end).copied().map_or(false, is_valid_cap_letter) {
        cap_end += 1;
    }
    if cap_end == i {
        return None;
    }
    let cap = core::str::from_utf8(&rep[i..cap_end]).expect("valid UTF-8 capture name");
    Some(CaptureRef {
        cap: match cap.parse::<usize>() {
            Ok(i) => Ref::Number(i),
            Err(_) => Ref::Named(cap),
        },
        end: cap_end,
    })
}
#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn memchr(n1: u8, haystack: &[u8]) -> Option<usize> {
    haystack.iter().position(|&b| b == n1)
}
