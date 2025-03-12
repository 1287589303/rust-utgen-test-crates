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
                if !first.starts_with(parser::ascii_alpha) || first.as_bytes()[1] != b':' {
                    return Err(());
                }

                string.push_str(first);
            }

            4 => {
                if !first.starts_with(parser::ascii_alpha) {
                    return Err(());
                }
                let bytes = first.as_bytes();
                if bytes[1] != b'%' || bytes[2] != b'3' || (bytes[3] != b'a' && bytes[3] != b'A') {
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

        // Currently non-unicode windows paths cannot be represented
        match percent_decode_str(segment).decode_utf8() {
            Ok(s) => string.push_str(&s),
            Err(..) => return Err(()),
        }
    }
    // ensure our estimated capacity was good
    if cfg!(test) {
        debug_assert!(
            string.len() <= estimated_capacity,
            "len: {}, capacity: {}",
            string.len(),
            estimated_capacity
        );
    }
    let path = PathBuf::from(string);
    debug_assert!(
        path.is_absolute(),
        "to_file_path() failed to produce an absolute Path"
    );
    Ok(path)
}