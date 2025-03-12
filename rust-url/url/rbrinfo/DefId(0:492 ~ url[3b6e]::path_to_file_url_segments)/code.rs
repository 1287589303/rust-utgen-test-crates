fn path_to_file_url_segments(
    path: &Path,
    serialization: &mut String,
) -> Result<(u32, HostInternal), ()> {
    use parser::SPECIAL_PATH_SEGMENT;
    use percent_encoding::percent_encode;
    #[cfg(target_os = "hermit")]
    use std::os::hermit::ffi::OsStrExt;
    #[cfg(any(unix, target_os = "redox"))]
    use std::os::unix::prelude::OsStrExt;
    if !path.is_absolute() {
        return Err(());
    }
    let host_end = to_u32(serialization.len()).unwrap();
    let mut empty = true;
    // skip the root component
    for component in path.components().skip(1) {
        empty = false;
        serialization.push('/');
        #[cfg(not(target_os = "wasi"))]
        serialization.extend(percent_encode(
            component.as_os_str().as_bytes(),
            SPECIAL_PATH_SEGMENT,
        ));
        #[cfg(target_os = "wasi")]
        serialization.extend(percent_encode(
            component.as_os_str().to_string_lossy().as_bytes(),
            SPECIAL_PATH_SEGMENT,
        ));
    }
    if empty {
        // An URL’s path must not be empty.
        serialization.push('/');
    }
    Ok((host_end, HostInternal::None))
}