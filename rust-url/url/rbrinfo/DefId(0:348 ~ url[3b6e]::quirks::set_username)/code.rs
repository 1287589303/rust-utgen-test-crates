pub fn set_username(url: &mut Url, new_username: &str) -> Result<(), ()> {
    url.set_username(new_username)
}