use std::ffi::c_char;

#[repr(C)]
#[no_mangle]
pub struct DownloadConfig {
    pub url: *const c_char,
    pub path: *const c_char,
    pub retry_times: u8,
    pub chunk_download: bool,
    pub download_in_memory: bool,
    pub version: i64,
    pub chunk_size: u64,
    pub timeout: u64,
}

#[no_mangle]
pub extern "C" fn DownloadConfig_Extern(config: DownloadConfig) {}