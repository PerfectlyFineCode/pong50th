use std::ffi::CString;
/// creates a const_c pointer from a string literal
#[macro_export]
macro_rules! const_c {
    ($c:expr) => {
        std::ffi::CString::new($c).unwrap().as_ptr()
    };
}