use std::ffi::{c_char, CStr, CString};
use std::fmt::{Display};

#[repr(C)]
#[derive(Clone)]
pub struct FFIString {
    pub ptr: *mut c_char,
    pub len: u32,
}

impl Display for FFIString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", unsafe { CStr::from_ptr(self.ptr).to_string_lossy().to_string() })
    }
}

impl From<String> for FFIString {
    fn from(value: String) -> Self {
        let len = value.as_bytes().len() as u32;
        let ptr = CString::new(value).unwrap().into_raw();
        FFIString {
            ptr,
            len,
        }
    }
}

impl From<&str> for FFIString {
    fn from(value: &str) -> Self {
        let len = value.as_bytes().len() as u32;
        let ptr = CString::new(value).unwrap().into_raw();
        FFIString {
            ptr,
            len,
        }
    }
}
