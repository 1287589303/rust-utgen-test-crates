pub fn set_password(url: &mut Url, new_password: &str) -> Result<(), ()> {
    url.set_password(if new_password.is_empty() {
        None
    } else {
        Some(new_password)
    })
}