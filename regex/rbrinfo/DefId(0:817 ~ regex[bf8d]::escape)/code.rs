pub fn escape(pattern: &str) -> alloc::string::String {
    regex_syntax::escape(pattern)
}