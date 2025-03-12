pub fn set_pathname(url: &mut Url, new_pathname: &str) {
    if url.cannot_be_a_base() {
        return;
    }
    if new_pathname.starts_with('/')
        || (SchemeType::from(url.scheme()).is_special()
            // \ is a segment delimiter for 'special' URLs"
            && new_pathname.starts_with('\\'))
    {
        url.set_path(new_pathname)
    } else if SchemeType::from(url.scheme()).is_special()
        || !new_pathname.is_empty()
        || !url.has_host()
    {
        let mut path_to_set = String::from("/");
        path_to_set.push_str(new_pathname);
        url.set_path(&path_to_set)
    } else {
        url.set_path(new_pathname)
    }
}