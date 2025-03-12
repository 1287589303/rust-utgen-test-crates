pub fn set_hash(url: &mut Url, new_hash: &str) {
    url.set_fragment(match new_hash {
        // If the given value is the empty string,
        // then set context object’s url’s fragment to null and return.
        "" => None,
        // Let input be the given value with a single leading U+0023 (#) removed, if any.
        _ if new_hash.starts_with('#') => Some(&new_hash[1..]),
        _ => Some(new_hash),
    })
}