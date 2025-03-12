fn _unsized_std_propagation() {
    check_t!(::std::path::Path);
    check_t!(::std::ffi::OsStr);
    check_t!(::std::ffi::CStr);
}