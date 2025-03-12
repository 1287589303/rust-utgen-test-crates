fn no_expansion<T: AsRef<[u8]>>(replacement: &T) -> Option<Cow<'_, [u8]>> {
    let replacement = replacement.as_ref();
    match crate::find_byte::find_byte(b'$', replacement) {
        Some(_) => None,
        None => Some(Cow::Borrowed(replacement)),
    }
}