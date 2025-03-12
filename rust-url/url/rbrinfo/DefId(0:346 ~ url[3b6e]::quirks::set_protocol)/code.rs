pub fn set_protocol(url: &mut Url, mut new_protocol: &str) -> Result<(), ()> {
    // The scheme state in the spec ignores everything after the first `:`,
    // but `set_scheme` errors if there is more.
    if let Some(position) = new_protocol.find(':') {
        new_protocol = &new_protocol[..position];
    }
    url.set_scheme(new_protocol)
}