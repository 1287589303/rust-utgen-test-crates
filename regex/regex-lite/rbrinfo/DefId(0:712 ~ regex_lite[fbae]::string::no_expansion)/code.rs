fn no_expansion<T: AsRef<str>>(t: &T) -> Option<Cow<'_, str>> {
    let s = t.as_ref();
    match s.find('$') {
        Some(_) => None,
        None => Some(Cow::Borrowed(s)),
    }
}