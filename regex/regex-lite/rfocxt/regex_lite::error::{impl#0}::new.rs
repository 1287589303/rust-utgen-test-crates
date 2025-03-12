#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Error {
    msg: &'static str,
}
impl Error {
    pub(crate) fn new(msg: &'static str) -> Error {
        Error { msg }
    }
}
