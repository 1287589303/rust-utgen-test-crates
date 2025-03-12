fn no_expansion<T: AsRef<str>>(replacement: &T) -> Option<Cow<'_, str>> {
    let replacement = replacement.as_ref();
    match crate::find_byte::find_byte(b'$', replacement.as_bytes()) {
        Some(_) => None,
        None => Some(Cow::Borrowed(replacement)),
    }
}