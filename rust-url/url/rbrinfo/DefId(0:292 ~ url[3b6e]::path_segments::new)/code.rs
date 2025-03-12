pub fn new(url: &mut Url) -> PathSegmentsMut<'_> {
    let after_path = url.take_after_path();
    let old_after_path_position = to_u32(url.serialization.len()).unwrap();
    // Special urls always have a non empty path
    if SchemeType::from(url.scheme()).is_special() {
        debug_assert!(url.byte_at(url.path_start) == b'/');
    } else {
        debug_assert!(
            url.serialization.len() == url.path_start as usize
                || url.byte_at(url.path_start) == b'/'
        );
    }
    PathSegmentsMut {
        after_first_slash: url.path_start as usize + "/".len(),
        url,
        old_after_path_position,
        after_path,
    }
}