pub use form_urlencoded;
use crate::host::HostInternal;
use crate::net::IpAddr;
#[cfg(feature = "std")]
#[cfg(any(unix, windows, target_os = "redox", target_os = "wasi", target_os = "hermit"))]
use crate::net::{SocketAddr, ToSocketAddrs};
use crate::parser::{to_u32, Context, Parser, SchemeType, USERINFO};
use alloc::borrow::ToOwned;
use alloc::str;
use alloc::string::{String, ToString};
use core::borrow::Borrow;
use core::convert::TryFrom;
use core::fmt::Write;
use core::ops::{Range, RangeFrom, RangeTo};
use core::{cmp, fmt, hash, mem};
use percent_encoding::utf8_percent_encode;
#[cfg(feature = "std")]
#[cfg(any(unix, windows, target_os = "redox", target_os = "wasi", target_os = "hermit"))]
use std::io;
#[cfg(feature = "std")]
use std::path::{Path, PathBuf};
pub use crate::host::Host;
pub use crate::origin::{OpaqueOrigin, Origin};
pub use crate::parser::{ParseError, SyntaxViolation};
pub use crate::path_segments::PathSegmentsMut;
pub use crate::slicing::Position;
pub use form_urlencoded::EncodingOverride;
#[cfg(feature = "std")]
#[cfg_attr(not(windows), allow(dead_code))]
fn file_url_segments_to_pathbuf_windows(
    estimated_capacity: usize,
    host: Option<&str>,
    mut segments: str::Split<'_, char>,
) -> Result<PathBuf, ()> {
    use percent_encoding::percent_decode_str;
    let mut string = String::new();
    string.try_reserve(estimated_capacity).map_err(|_| ())?;
    if let Some(host) = host {
        string.push_str(r"\\");
        string.push_str(host);
    } else {
        let first = segments.next().ok_or(())?;
        match first.len() {
            2 => {
                if !first.starts_with(parser::ascii_alpha) || first.as_bytes()[1] != b':'
                {
                    return Err(());
                }
                string.push_str(first);
            }
            4 => {
                if !first.starts_with(parser::ascii_alpha) {
                    return Err(());
                }
                let bytes = first.as_bytes();
                if bytes[1] != b'%' || bytes[2] != b'3'
                    || (bytes[3] != b'a' && bytes[3] != b'A')
                {
                    return Err(());
                }
                string.push_str(&first[0..1]);
                string.push(':');
            }
            _ => return Err(()),
        }
    };
    for segment in segments {
        string.push('\\');
        match percent_decode_str(segment).decode_utf8() {
            Ok(s) => string.push_str(&s),
            Err(..) => return Err(()),
        }
    }
    if cfg!(test) {
        debug_assert!(
            string.len() <= estimated_capacity, "len: {}, capacity: {}", string.len(),
            estimated_capacity
        );
    }
    let path = PathBuf::from(string);
    debug_assert!(
        path.is_absolute(), "to_file_path() failed to produce an absolute Path"
    );
    Ok(path)
}
