use std::ffi::{c_char, CString};

#[repr(C)]
#[derive(Clone)]
pub struct UTF16String {
    pub ptr: *const u16,
    pub len: u32,
}

impl Into<String> for UTF16String {
    fn into(self) -> String {
        let slice = unsafe { std::slice::from_raw_parts(self.ptr, self.len as usize) };
        String::from_utf16(slice).unwrap()
    }
}

#[no_mangle]
pub extern "C" fn UTF16String_Extern(_: UTF16String) {}

#[no_mangle]
pub unsafe extern "C" fn free_c_string(str: *mut c_char) {
    unsafe { CString::from_raw(str) };
}