fn file_url_segments_to_pathbuf(
    estimated_capacity: usize,
    host: Option<&str>,
    segments: str::Split<'_, char>,
) -> Result<PathBuf, ()> {
    use alloc::vec::Vec;
    use percent_encoding::percent_decode;
    #[cfg(not(target_os = "wasi"))]
    use std::ffi::OsStr;
    #[cfg(target_os = "hermit")]
    use std::os::hermit::ffi::OsStrExt;
    #[cfg(any(unix, target_os = "redox"))]
    use std::os::unix::prelude::OsStrExt;

    if host.is_some() {
        return Err(());
    }

    let mut bytes = Vec::new();
    bytes.try_reserve(estimated_capacity).map_err(|_| ())?;
    if cfg!(target_os = "redox") {
        bytes.extend(b"file:");
    }

    for segment in segments {
        bytes.push(b'/');
        bytes.extend(percent_decode(segment.as_bytes()));
    }

    // A windows drive letter must end with a slash.
    if bytes.len() > 2
        && bytes[bytes.len() - 2].is_ascii_alphabetic()
        && matches!(bytes[bytes.len() - 1], b':' | b'|')
    {
        bytes.push(b'/');
    }

    #[cfg(not(target_os = "wasi"))]
    let path = PathBuf::from(OsStr::from_bytes(&bytes));
    #[cfg(target_os = "wasi")]
    let path = String::from_utf8(bytes)
        .map(|path| PathBuf::from(path))
        .map_err(|_| ())?;

    debug_assert!(
        path.is_absolute(),
        "to_file_path() failed to produce an absolute Path"
    );

    Ok(path)
}