fn posix_class(
    kind: &str,
) -> Result<impl Iterator<Item = hir::ClassRange>, Error> {
    let slice: &'static [(u8, u8)] = match kind {
        "alnum" => &[(b'0', b'9'), (b'A', b'Z'), (b'a', b'z')],
        "alpha" => &[(b'A', b'Z'), (b'a', b'z')],
        "ascii" => &[(b'\x00', b'\x7F')],
        "blank" => &[(b'\t', b'\t'), (b' ', b' ')],
        "cntrl" => &[(b'\x00', b'\x1F'), (b'\x7F', b'\x7F')],
        "digit" => &[(b'0', b'9')],
        "graph" => &[(b'!', b'~')],
        "lower" => &[(b'a', b'z')],
        "print" => &[(b' ', b'~')],
        "punct" => &[(b'!', b'/'), (b':', b'@'), (b'[', b'`'), (b'{', b'~')],
        "space" => &[
            (b'\t', b'\t'),
            (b'\n', b'\n'),
            (b'\x0B', b'\x0B'),
            (b'\x0C', b'\x0C'),
            (b'\r', b'\r'),
            (b' ', b' '),
        ],
        "upper" => &[(b'A', b'Z')],
        "word" => &[(b'0', b'9'), (b'A', b'Z'), (b'_', b'_'), (b'a', b'z')],
        "xdigit" => &[(b'0', b'9'), (b'A', b'F'), (b'a', b'f')],
        _ => return Err(Error::new(ERR_POSIX_CLASS_UNRECOGNIZED)),
    };
    Ok(slice.iter().map(|&(start, end)| hir::ClassRange {
        start: char::from(start),
        end: char::from(end),
    }))
}